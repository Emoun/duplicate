//! This crate provides macros for easy code duplication with substitution:
//!
//! - [`duplicate_item`]: Attribute macro.
//! - [`duplicate`]: Function-like procedural macro.
//!
//! The only major difference between the two is where you can use them.
//! Therefore, the following section presents how to use
//! [`duplicate_item`] only. Refer to [`duplicate`]'s documentation for how it
//! defers from what is specified below.
//!
//! [`duplicate_item`]: attr.duplicate_item.html
//! [`duplicate`]: macro.duplicate.html
//! # Usage
//!
//! Say you have a trait with a method `is_max` that should return `true` if the
//! value of the object is the maximum allowed and `false` otherwise:
//! ```
//! trait IsMax {
//!   fn is_max(&self) -> bool;
//! }
//! ```
//! You would like to implement this trait for the three integer types `u8`,
//! `u16`, and `u32`:
//!
//! ```
//! # trait IsMax {fn is_max(&self) -> bool;}
//! impl IsMax for u8 {
//!   fn is_max(&self) -> bool {
//!     *self == 255
//!   }
//! }
//! impl IsMax for u16 {
//!   fn is_max(&self) -> bool {
//!     *self == 65_535
//!   }
//! }
//! impl IsMax for u32 {
//!   fn is_max(&self) -> bool {
//!     *self == 4_294_967_295
//!   }
//! }
//! ```
//! This is a lot of repetition. Only the type and the maximum value are
//! actually different between the three implementations. This might not be much
//! in our case, but imagine doing this for all the integer types (10, as of the
//! last count.) We can use the `duplicate_item` attribute to avoid repeating
//! ourselves:
//!
//! ```
//! # trait IsMax {fn is_max(&self) -> bool;}
//! use duplicate::duplicate_item;
//! #[duplicate_item(
//!   int_type  max_value;
//!   [ u8 ]    [ 255 ];
//!   [ u16 ]   [ 65_535 ];
//!   [ u32 ]   [ 4_294_967_295 ];
//! )]
//! impl IsMax for int_type {
//!   fn is_max(&self) -> bool {
//!     *self == max_value
//!   }
//! }
//!
//! assert!(!42u8.is_max());
//! assert!(!42u16.is_max());
//! assert!(!42u32.is_max());
//! ```
//! The above code will expand to the three implementations before it.
//! The attribute invocation specifies that the following item should be
//! substituted by three duplicates of itself. Additionally, each occurrence of
//! the identifier `int_type` in the first duplicate should be replaced by `u8`,
//! in the second duplicate by `u16`, and in the last by `u32`. Likewise, each
//! occurrence of `max_value` should be replaced by `255`, `65_535`, and
//! `4_294_967_295` in the first, second, and third duplicates respectively.
//!
//! `int_type` and `max_value` are called _substitution identifiers_, while `[
//! u8 ]`, `[ u16 ]`, and `[ u32 ]` are each _substitutions_ for `int_type` and
//! `[255]`, `[65_535]`, and `[4_294_967_295]` are substitutions for
//! `max_value`. Each pair of substitutions for the identifiers is called a
//! _substitution group_. Substitution groups must be seperated by `;` and the
//! number of duplicates made is equal to the number of subsitution groups.
//!
//! Substitution identifiers must be valid Rust identifiers.
//! The code inside substitutions can be arbitrary, as long as the expanded code
//! is valid.
//!
//! ## Parameterized Substitution
//!
//! Say we have a struct that wraps a vector and we want to give
//! access to the vector's `get` and `get_mut` methods directly:
//!
//! ```
//! struct VecWrap<T>(Vec<T>);
//!
//! impl<T> VecWrap<T> {
//!   pub fn get(&self, idx: usize) -> Option<&T> {
//!     self.0.get(idx)
//!   }
//!   pub fn get_mut(&mut self, idx: usize) -> Option<&mut T> {
//!     self.0.get_mut(idx)
//!   }
//! }
//!
//! let mut vec = VecWrap(vec![1,2,3]);
//! assert_eq!(*vec.get(0).unwrap(), 1);
//! *vec.get_mut(1).unwrap() = 5;
//! assert_eq!(*vec.get(1).unwrap(), 5);
//! ```
//!
//! Even though the implementations of the two versions of `get` are almost
//! identical, we will always need to duplicate the code, because Rust cannot be
//! generic over mutability. _Parameterized substitution_ allows us to pass code
//! snippets to substitution identifiers to customize the substitution for that
//! specific use of the identifier. We can use it to help with the
//! implementation of constant and mutable versions of methods and functions.
//! The following `impl` is identical to the above code:
//!
//! ```
//! # use duplicate::duplicate_item;
//! # struct VecWrap<T>(Vec<T>);
//! impl<T> VecWrap<T> {
//!   #[duplicate_item(
//!     method     reference(type);
//!     [get]      [& type];
//!     [get_mut]  [&mut type];
//!   )]
//!   pub fn method(self: reference([Self]), idx: usize) -> Option<reference([T])> {
//!     self.0.method(idx)
//!   }
//! }
//! # let mut vec = VecWrap(vec![1,2,3]);
//! # assert_eq!(*vec.get(0).unwrap(), 1);
//! # *vec.get_mut(1).unwrap() = 5;
//! # assert_eq!(*vec.get(1).unwrap(), 5);
//! ```
//!
//! In a `duplicate_item` invocation, if a substitution identifier is followed
//! by parenthises containing a list of parameters, they can be used in the
//! substitution. In this example, the `reference` identifier takes 1 parameter
//! named `type`, which is used in the substitutions to create either a shared
//! reference to the type or a mutable one. When using the `reference` in the
//! method declaration, we give it different types as arguments to construct
//! either shared or mutable references.
//! E.g. `reference([Self])` becomes `&Self` in the first duplicate and `&mut
//! Self` in the second. An argument can be any code snippet inside `[]`.
//!
//! A substitution identifier can take any number of parameters.
//! We can use this if we need to also provide the references with a lifetime:
//!
//! ```
//! # use duplicate::duplicate_item;
//! # struct VecWrap<T>(Vec<T>);
//! impl<T> VecWrap<T> {
//!   #[duplicate_item(
//!     method     reference(lifetime, type);
//!     [get]      [& 'lifetime type];
//!     [get_mut]  [& 'lifetime mut type];
//!   )]
//!   pub fn method<'a>(self: reference([a],[Self]),idx: usize) -> Option<reference([a],[T])> {
//!     self.0.method(idx)
//!   }
//! }
//! # let mut vec = VecWrap(vec![1,2,3]);
//! # assert_eq!(*vec.get(0).unwrap(), 1);
//! # *vec.get_mut(1).unwrap() = 5;
//! # assert_eq!(*vec.get(1).unwrap(), 5);
//! ```
//!
//! Here we pass the lifetime `'a` to the substitution as the first argument,
//! and the type as the second. Notice how the arguments are separated by a
//! comma. This results in the following code:
//!
//! ```
//! # struct VecWrap<T>(Vec<T>);
//! impl<T> VecWrap<T> {
//!   pub fn get<'a>(self: &'a Self, idx: usize) -> Option<&'a T> {
//!     self.0.get(idx)
//!   }
//!   pub fn get_mut<'a>(self: &'a mut Self, idx: usize) -> Option<&'a mut T> {
//!     self.0.get_mut(idx)
//!   }
//! }
//! # let mut vec = VecWrap(vec![1,2,3]);
//! # assert_eq!(*vec.get(0).unwrap(), 1);
//! # *vec.get_mut(1).unwrap() = 5;
//! # assert_eq!(*vec.get(1).unwrap(), 5);
//! ```
//!
//! Notice also the way we pass lifetimes to identifiers: `reference([a],
//! [Self])`. The lifetime is passed without the `'` prefix, which is instead
//! present in the substitution before the 	lifetime: `[& 'lifetime type]`.
//! This is because the rust syntax disallows lifetimes in brackets on their
//! own. Our solution is therefore a hacking of the system and not a property of
//! `duplicate_item` itself.
//!
//! ## Nested Invocation
//!
//! Imagine we have the following trait with the method `is_negative` that
//! should return `true` if the value of the object is negative and `false`
//! otherwise:
//! ```
//! trait IsNegative {
//!   fn is_negative(&self) -> bool;
//! }
//! ```
//! We want to implement this for the six integer types `u8`, `u16`, `u32`,
//! `i8`, `i16`, and `i32`. For the first three types, which are all unsigned,
//! the implementation of this trait should trivially return `false` as they
//! can't be negative. However, for the remaining, signed types their
//! implementations is identical (checking whether they are less than `0`), but,
//! of course, different from the first three:
//! ```
//! # trait IsNegative { fn is_negative(&self) -> bool;}
//! impl IsNegative for u8 {
//!   fn is_negative(&self) -> bool {
//!     false
//!   }
//! }
//! impl IsNegative for u16 {
//!   fn is_negative(&self) -> bool {
//!     false
//!   }
//! }
//! impl IsNegative for u32 {
//!   fn is_negative(&self) -> bool {
//!     false
//!   }
//! }
//! impl IsNegative for i8 {
//!   fn is_negative(&self) -> bool {
//!     *self < 0
//!   }
//! }
//! impl IsNegative for i16 {
//!   fn is_negative(&self) -> bool {
//!     *self < 0
//!   }
//! }
//! impl IsNegative for i32 {
//!   fn is_negative(&self) -> bool {
//!     *self <  0
//!   }
//! }
//! # assert!(!42u8.is_negative());
//! # assert!(!42u16.is_negative());
//! # assert!(!42u32.is_negative());
//! # assert!(!42i8.is_negative());
//! # assert!(!42i16.is_negative());
//! # assert!(!42i32.is_negative());
//! ```
//!
//! Notice how the code repetition is split over 2 axes: 1) They all implement
//! the same trait 2) the method implementations of the first 3 are identical to
//! each other but different to the next 3, which are also mutually identical.
//! To implement this using only the syntax we have already seen, we could do
//! something like this:
//!
//! ```
//! # trait IsNegative { fn is_negative(&self) -> bool;}
//! # use duplicate::duplicate_item;
//! #[duplicate_item(
//!   int_type implementation;
//!   [u8]     [false];
//!   [u16]    [false];
//!   [u32]    [false];
//!   [i8]     [*self < 0];
//!   [i16]    [*self < 0];
//!   [i32]    [*self < 0]
//! )]
//! impl IsNegative for int_type {
//!   fn is_negative(&self) -> bool {
//!     implementation
//!   }
//! }
//!
//! assert!(!42u8.is_negative());
//! assert!(!42u16.is_negative());
//! assert!(!42u32.is_negative());
//! assert!(!42i8.is_negative());
//! assert!(!42i16.is_negative());
//! assert!(!42i32.is_negative());
//! ```
//!
//! However, ironically, we here had to repeat ourselves in the macro invocation
//! instead of the code: we needed to repeat the implementations `[ false ]` and
//! `[ *self < 0 ]` three times each. We can utilize _nested invocation_ to
//! remove the last bit of repetition:
//!
//! ```
//! # trait IsNegative { fn is_negative(&self) -> bool;}
//! # use duplicate::duplicate_item;
//! #[duplicate_item(
//!   int_type implementation;
//!   duplicate!{
//!     [
//!       int_type_nested; [u8]; [u16]; [u32]
//!     ]
//!     [ int_type_nested ] [ false ];
//!   }
//!   duplicate!{
//!     [
//!       int_type_nested; [i8]; [i16]; [i32]
//!     ]
//!     [ int_type_nested ] [ *self < 0 ];
//!   }
//! )]
//! impl IsNegative for int_type {
//!   fn is_negative(&self) -> bool {
//!     implementation
//!   }
//! }
//!
//! assert!(!42u8.is_negative());
//! assert!(!42u16.is_negative());
//! assert!(!42u32.is_negative());
//! assert!(!42i8.is_negative());
//! assert!(!42i16.is_negative());
//! assert!(!42i32.is_negative());
//! ```
//!
//! We use `duplicate!{..}` to invoke the macro inside itself.
//! In our example, we have 2 invocations that each produce 3 substitution
//! groups, inserting the correct `implementation` for their signed or unsigned
//! types. The above nested invocation is equivalent to the previous, non-nested
//! invocation, and actually expands to it as an intermediate step before
//! expanding the outer-most invocation.
//!
//! Deeper levels of nested invocation are possible and work as expected.
//! There is no limit on the depth of nesting, however, as might be clear from
//! our example, it can get complicated to read.
//!
//! Lastly, we should note that we can have nested invocations interleaved with
//! normal substitution groups. For example, say we want to implement
//! `IsNegative` for `i8`, but don't want the same for `i16` and `i32`. We could
//! do the following:
//!
//! ```
//! # trait IsNegative { fn is_negative(&self) -> bool;}
//! # use duplicate::duplicate_item;
//! #[duplicate_item(
//!   int_type implementation;
//!   duplicate!{
//!     [                                     // -+
//!       int_type_nested; [u8]; [u16]; [u32] //  | Nested invocation producing 3
//!     ]                                     //  | substitution groups
//!     [int_type_nested ] [ false ];         //  |
//!   }                                       // -+
//!   [ i8 ] [ *self < 0 ]                    // -- Substitution group 4
//! )]
//! impl IsNegative for int_type {
//!   fn is_negative(&self) -> bool {
//!     implementation
//!   }
//! }
//!
//! # assert!(!42u8.is_negative());
//! # assert!(!42u16.is_negative());
//! # assert!(!42u32.is_negative());
//! # assert!(!42i8.is_negative());
//! ```
//!
//! In general, nested invocations can be used anywhere. However, note that
//! nested invocations are only recognized by the identifier `duplicate`,
//! followed by `!`, followed by a delimiter within which the nested invocation
//! is. Therefore, care must be taken to ensure the surrounding code is correct
//! after the expansion. E.g. maybe `;` is needed after the invocation, or
//! commas must be produced by the nested invocation itself as part of a list.
//!
//! ## Verbose Syntax
//!
//! The syntax used in the previous examples is the _short syntax_.
//! `duplicate_item` also accepts a _verbose syntax_ that is less concise, but
//! more readable in some circumstances. Using the verbose syntax, the very
//! first example above looks like this:
//!
//! ```
//! # trait IsMax {fn is_max(&self) -> bool;}
//! use duplicate::duplicate_item;
//! #[duplicate_item(
//!   [
//!     int_type  [ u8 ]
//!     max_value [ 255 ]
//!   ]
//!   [
//!     int_type  [ u16 ]
//!     max_value [ 65_535 ]
//!   ]
//!   [
//!     int_type  [ u32 ]
//!     max_value [ 4_294_967_295 ]
//!   ]
//! )]
//! impl IsMax for int_type {
//!   fn is_max(&self) -> bool {
//!     *self == max_value
//!   }
//! }
//!
//! # assert!(!42u8.is_max());
//! # assert!(!42u16.is_max());
//! # assert!(!42u32.is_max());
//! ```
//!
//! In the verbose syntax, a substitution group is put inside '[]' and
//! includes a list of substitution identifiers followed by their substitutions.
//! No `;`s are needed. Here is an annotated version of the same code:
//!
//! ```
//! # trait IsMax {fn is_max(&self) -> bool;}
//! # use duplicate::duplicate_item;
//! #[duplicate_item(
//!   [                               //-+
//!     int_type  [ u8 ]              // | Substitution group 1
//!     max_value [ 255 ]             // |
//! //  ^^^^^^^^^ ^^^^^^^ substitution   |
//! //  |                                |
//! //  substitution identifier          |
//!   ]                               //-+
//!   [                               //-+
//!     int_type  [ u16 ]             // | Substitution group 2
//!     max_value [ 65_535 ]          // |
//!   ]                               //-+
//!   [                               //-+
//!     max_value [ 4_294_967_295 ]   // | Substitution group 3
//!     int_type  [ u32 ]             // |
//!   ]                               //-+
//! )]
//! # impl IsMax for int_type {
//! #  fn is_max(&self) -> bool {
//! #     *self == max_value
//! #    }
//! #  }
//! #
//! # assert!(!42u8.is_max());
//! # assert!(!42u16.is_max());
//! # assert!(!42u32.is_max());
//! ```
//! Note that in each substitution group every identifier must have exactly one
//! substitution. All the groups must have the exact same identifiers, though
//! the order in which they arrive in each group is not important. For example,
//! in the annotated example, the third group has the `max_value` identifier
//! before `int_type` without having any effect on the expanded code.
//!
//! The verbose syntax is not very concise but it has some advantages over
//! the short syntax in regards to readability. Using many identifiers and
//! long substitutions can quickly become unwieldy in the short syntax.
//! The verbose syntax deals better with both cases as it will grow horizontally
//! instead of vertically.
//!
//! The verbose syntax also offers nested invocation. The syntax is exactly the
//! same, but since there is no initial substitution identifier list, nested
//! calls can be used anywhere (though still not inside substitution groups.)
//! The previous `IsNegative` nested invocation example can be written as
//! follows:
//!
//! ```
//! # trait IsNegative { fn is_negative(&self) -> bool;}
//! # use duplicate::duplicate_item;
//! #[duplicate_item(
//!   duplicate!{
//!     [ int_type_nested; [u8]; [u16]; [u32] ]
//!     [
//!       int_type [ int_type_nested ]
//!       implementation [ false ]
//!     ]
//!   }
//!   duplicate!{
//!     [ int_type_nested; [i8]; [i16]; [i32] ]
//!     [
//!       int_type [ int_type_nested ]
//!       implementation [ *self < 0 ]
//!     ]
//!   }
//! )]
//! impl IsNegative for int_type {
//!   fn is_negative(&self) -> bool {
//!     implementation
//!   }
//! }
//!
//! assert!(!42u8.is_negative());
//! assert!(!42u16.is_negative());
//! assert!(!42u32.is_negative());
//! assert!(!42i8.is_negative());
//! assert!(!42i16.is_negative());
//! assert!(!42i32.is_negative());
//! ```
//!
//! It's important to notice that the nested invocation doesn't know it
//! isn't the outer-most invocation and therefore doesn't discriminate between
//! identifiers. We had to use a different identifier in the nested invocations
//! (`int_type_nested`) than in the code (`int_type`), because otherwise the
//! nested invocation would substitute the substitution identifier too, instead
//! of only substituting in the nested invocation's substitute.
//!
//! Nested invocations must produce the syntax of their
//! parent invocation. However, each invocation's private syntax is free
//! to use any syntax type. Notice in our above example, the nested
//! invocations use short syntax but produce verbose syntax for the outer-most
//! invocation.
//!
//! ## Global Substitutions
//!
//! Say we have a function that takes two types as inputs and returns the same
//! types as output:
//!
//! ```
//! # struct Some<T1,T2>(T1,T2);
//! # struct Complex<T>(T);
//! # struct Type<T>(T);
//! # struct WeDont<T1,T2,T3>(T1,T2,T3);
//! # struct Want();
//! # struct To();
//! # struct Repeat();
//! # struct Other();
//! fn some_func(
//!   arg1: Some<Complex<()>, Type<WeDont<Want, To, Repeat>>>,
//!   arg2: Some<Other, Complex<Type<(To, Repeat)>>>)
//!   -> (
//!     Some<Complex<()>, Type<WeDont<Want, To, Repeat>>>,
//!     Some<Other, Complex<Type<(To, Repeat)>>>
//!   )
//! {
//!  # /*
//!   ...
//!  # */
//!  # unimplemented!()
//! }
//! ```
//!
//! Using global substitution, we can avoid repeating the types:
//!
//! ```
//! # struct Some<T1,T2>(T1,T2);
//! # struct Complex<T>(T);
//! # struct Type<T>(T);
//! # struct WeDont<T1,T2,T3>(T1,T2,T3);
//! # struct Want();
//! # struct To();
//! # struct Repeat();
//! # struct Other();
//! # use duplicate::duplicate_item;
//! #[duplicate_item(
//!   typ1 [Some<Complex<()>, Type<WeDont<Want, To, Repeat>>>];
//!   typ2 [Some<Other, Complex<Type<(To, Repeat)>>>];
//! )]
//! fn some_func(arg1: typ1, arg2: typ2) -> (typ1, typ2){
//!  # /*
//!   ...
//!  # */
//!  # unimplemented!()
//! }
//! ```
//!
//! Here we have defined the two global substitution variables `typ1` and
//! `typ2`, and used them in the function definition. Global substitutions have
//! the same syntax as verbose syntax substitution (identifier, optionally
//! followed by parameters, followed by a substitution.) In our example, no
//! short or verbose syntax substitution groups are given. While this is not
//! usually allowed, since we have given at least one global substitution, the
//! item will simply be kept as is, except with the global substitutions.
//!
//! We can follow global substitutions by substitution groups to achieve
//! duplication too:
//!
//! ```
//! # struct Some<T1,T2>(T1,T2);
//! # struct Complex<T>(T);
//! # struct Type<T>(T);
//! # struct WeDont<T1,T2,T3>(T1,T2,T3);
//! # struct Want();
//! # struct To();
//! # struct Repeat();
//! # struct Other();
//! # use duplicate::duplicate_item;
//! #[duplicate_item(
//!   typ1 [Some<Complex<()>, Type<WeDont<Want, To, Repeat>>>];
//!   typ2 [Some<Other, Complex<Type<(To, Repeat)>>>];
//!   method     reference(type);
//!   [get]      [& type];
//!   [get_mut]  [&mut type];
//! )]
//! fn method(
//!   arg0: reference([Type<()>]),
//!   arg1: typ1,
//!   arg2: typ2)
//!   -> (reference([typ1]), reference([typ2]))
//! {
//!  # /*
//!   ...
//!  # */
//!  # unimplemented!()
//! }
//! ```
//!
//! Here we duplicate the function to use either shared or mutable reference,
//! while reusing `typ1` and `typ2` in both duplicates.
//!
//! The following additional rules apply when using global substitutions:
//!
//! * All global substitutions must come before any short or verbose syntax
//!   substitution groups.
//! * Global substitution variable are __not__ substituted inside the bodies of
//!   following substitutions. If that is needed, multiple invocations can be
//!   used.
//! * All global substitutions must be separated by `;`, also when followed by
//!   substitution groups.
//!
//! # Crate Features
//!
//! ### `module_disambiguation`
//! __Implicit Module Name Disambiguation__ (Enabled by default)
//!
//! It is sometime beneficial to apply `duplicate_item` to a module, such that
//! all its contents are duplicated at once. However, this will always need the
//! resulting modules to have unique names to avoid the compiler issueing an
//! error. Without `module_disambiguation`, module names must be substituted
//! manually. With `module_disambiguation`, the following will compile
//! successfully:
//!
//! ```
//! # #[cfg(feature="module_disambiguation")] // Ensure test is only run if feature is on
//! # {
//! # use duplicate::duplicate_item;
//! #[duplicate_item(
//!   int_type  max_value;
//!   [ u8 ]    [ 255 ];
//!   [ u16 ]   [ 65_535 ];
//!   [ u32 ]   [ 4_294_967_295 ];
//! )]
//! mod module {
//! # // There is a bug with rustdoc, where these traits cannot
//! # // be imported using 'use super::*'.
//! # // This is a workaround
//! # pub trait IsNegative { fn is_negative(&self) -> bool;}
//! # pub trait IsMax {fn is_max(&self) -> bool;}
//!   impl IsMax for int_type {
//!     fn is_max(&self) -> bool {
//!       *self == max_value
//!     }
//!   }
//!   impl IsNegative for int_type {
//!     fn is_negative(&self) -> bool {
//!       false
//!     }
//!   }
//! }
//! # // This is part of the workaround for not being able to import
//! # // these traits in each module. We rename them so that they
//! # // don't clash with each other.
//! # use module_u8::IsNegative as trait1;
//! # use module_u8::IsMax as trait2;
//! # use module_u16::IsNegative as trait3;
//! # use module_u16::IsMax as trait4;
//! # use module_u32::IsNegative as trait5;
//! # use module_u32::IsMax as trait6;
//!
//! assert!(!42u8.is_max());
//! assert!(!42u16.is_max());
//! assert!(!42u32.is_max());
//! assert!(!42u8.is_negative());
//! assert!(!42u16.is_negative());
//! assert!(!42u32.is_negative());
//! # }
//! ```
//!
//! This works because the three duplicate modules get assigned unique names:
//! `module_u8`, `module_u16`, and `module_u32`. However, this only works if a
//! substitution identifier can be found, where all its substitutions only
//! produce a single identifier and nothing else. Those identifiers are then
//! converted to snake case, and postfixed to the original module's name,
//! e.g., `module  + u8 = module_u8`. The first suitable substitution
//! identifier is chosen.
//!
//! Notes:
//!
//! * The exact way unique names are generated is not part of any stability
//!   guarantee and should not be depended upon. It may change in the future
//!   without bumping the major version.
//! * Only the name of the module is substituted with the disambiguated name.
//!   Any matching identifier in the body of the module is ignored.
//!
//! ### `pretty_errors`
//! __More Detailed Error Messages__ (Enabled by default)
//!
//! Enabling this feature will make error messages indicate exactly where the
//! offending code is. Without this feature, error messages will not provide
//! detailed location indicators for errors.
//!
//! This feature is has no effect on expansion. Therefore, libraries are advised
//! to keep this feature off (note that it's enabled by default)
//! to avoid forcing it on users.
//!
//! # Disclaimer
//!
//! This crate does not try to justify or condone the usage of code duplication
//! instead of proper abstractions.
//! This crate should only be used where it is not possible to reduce code
//! duplication through other means, or where it simply is not worth it.
//!
//! As an example, libraries that have two or more structs/traits with similar
//! APIs might use this macro to test them without having to copy-paste test
//! cases and manually make the needed edits.
#![cfg_attr(feature = "fail-on-warnings", deny(warnings))]

extern crate proc_macro;

mod crate_readme_test;
mod error;
#[cfg(feature = "module_disambiguation")]
mod module_disambiguation;
mod parse;
mod pretty_errors;
mod substitute;
mod token_iter;

use crate::{
	error::Error,
	token_iter::{is_ident, Token, TokenIter},
};
use parse::*;
use proc_macro::{Delimiter, Group, Ident, Span, TokenStream};
#[cfg(feature = "pretty_errors")]
use proc_macro_error::{abort, proc_macro_error};
use std::{collections::HashMap, iter::empty};
use substitute::*;

/// Duplicates the item and substitutes specific identifiers for different code
/// snippets in each duplicate.
///
/// # Short Syntax
/// ```
/// use duplicate::duplicate_item;
/// trait IsMax {
///   fn is_max(&self) -> bool;
/// }
///
/// #[duplicate_item(
///   int_type  max_value;
///   [ u8 ]    [ 255 ];
///   [ u16 ]   [ 65_535 ];
///   [ u32 ]   [ 4_294_967_295 ];
/// )]
/// impl IsMax for int_type {
///   fn is_max(&self) -> bool {
///     *self == max_value
///   }
/// }
///
/// assert!(!42u8.is_max());
/// assert!(!42u16.is_max());
/// assert!(!42u32.is_max());
/// ```
/// The implementation of `IsMax` is duplicated 3 times:
///
/// 1. For the type `u8` and the its maximum value `255`.
/// 2. For the type `u16` and the its maximum value `65_535 `.
/// 3. For the type `u32` and the its maximum value `4_294_967_295 `.
///
/// This syntax must start with a list of all identifiers followed by `;`.
/// Then a `;` seperated list of substitution groups must be given (at least 1
/// group). Every group is a list of substitutions, one for each substitution
/// identifier given in the first line.
/// The substitutions must be enclosed in `[]` but are otherwise
/// free.
///
/// # Verbose Syntax
///
/// ```
/// use duplicate::duplicate_item;
/// trait IsMax {
///   fn is_max(&self) -> bool;
/// }
///
/// #[duplicate_item(
///   [
///     int_type  [ u8 ]
///     max_value [ 255 ]
///   ]
///   [
///     int_type  [ u16 ]
///     max_value [ 65_535 ]
///   ]
///   [
///     max_value [ 4_294_967_295 ]
///     int_type  [ u32 ]
///   ]
/// )]
/// impl IsMax for int_type {
///   fn is_max(&self) -> bool {
///     *self == max_value
///   }
/// }
///
/// assert!(!42u8.is_max());
/// assert!(!42u16.is_max());
/// assert!(!42u32.is_max());
/// ```
/// Has the same functionality as the previous short-syntax example.
///
/// For each duplicate needed, a _substitution group_ must be given enclosed in
/// `[]`. A substitution group is a set of identifiers and
/// substitution pairs, like in the short syntax, but there can only be one
/// substitution per identifier. All substitution groups must have the same
/// identifiers, however their order is unimportant, as can be seen from the
/// last substitution group above, where `max_value` comes before `int_type`.
///
/// # Parameterized Substitutoin
///
/// ```
/// use duplicate::duplicate_item;
/// struct VecWrap<T>(Vec<T>);
///
/// impl<T> VecWrap<T> {
///   #[duplicate_item(
///     method     reference(lifetime, type);
///     [get]      [& 'lifetime type];
///     [get_mut]  [& 'lifetime mut type];
///   )]
///   pub fn method<'a>(self: reference([a],[Self]),idx: usize) -> Option<reference([a],[T])> {
///     self.0.method(idx)
///   }
/// }
///
/// let mut vec = VecWrap(vec![1,2,3]);
/// assert_eq!(*vec.get(0).unwrap(), 1);
/// *vec.get_mut(1).unwrap() = 5;
/// assert_eq!(*vec.get(1).unwrap(), 5);
/// ```
///
/// This implements two versions of the method:
///
/// - `get`: Borrows `self` immutably and return a shared reference.
/// - `get_mut`: Borrows `self` mutably and returns a mutable reference.
///
/// If an identifier is followed by parenthises (in both its declaration and its
/// use), a set of parameters can be provided to customize the subtituion for
/// each use. In the declaration a list of identifiers is given, which can be
/// used in its substitutions. When using the identifier, argument code snippets
/// must be given in a comma separated list, with each argument being inclosed
/// in `[]`.
///
/// Parameterized substitution is also available for the verbose syntax:
///
/// ```
/// # use duplicate::duplicate_item;
/// # struct VecWrap<T>(Vec<T>);
/// impl<T> VecWrap<T> {
///   #[duplicate_item(
///     [
///       method                     [get]
///       reference(lifetime, type)  [& 'lifetime type]
///     ]
///     [
///       method                     [get_mut]
///       reference(lifetime, type)  [& 'lifetime mut type]
///     ]
///   )]
///   pub fn method<'a>(self: reference([a],[Self]),idx: usize) -> Option<reference([a],[T])> {
///     self.0.method(idx)
///   }
/// }
/// # let mut vec = VecWrap(vec![1,2,3]);
/// # assert_eq!(*vec.get(0).unwrap(), 1);
/// # *vec.get_mut(1).unwrap() = 5;
/// # assert_eq!(*vec.get(1).unwrap(), 5);
/// ```
///
/// # Nested Invocation
/// ```
/// use duplicate::duplicate_item;
/// trait IsNegative {
///   fn is_negative(&self) -> bool;
/// }
///
/// #[duplicate_item(
///   int_type implementation;
///   duplicate!{
///     [                                  // -+
///       int_type_nested;[u8];[u16];[u32] //  | Nested invocation producing 3
///     ]                                  //  | substitution groups
///     [ int_type_nested ] [ false ];     //  |
///   }                                    // -+
///   [ i8 ] [ *self < 0 ]                 // -- Substitution group 4
/// )]
/// impl IsNegative for int_type {
///   fn is_negative(&self) -> bool {
///     implementation
///   }
/// }
///
/// assert!(!42u8.is_negative());
/// assert!(!42u16.is_negative());
/// assert!(!42u32.is_negative());
/// assert!(!42i8.is_negative());
/// ```
///
/// This implements `IsNegative` 4 times:
///
/// 1. For the type `u8` with the implementation of the method simply returning
/// `false`. 1. For the type `u16` the same way as `u8`.
/// 1. For the type `u32` the same way as `u8` and `u16`.
/// 1. For `i8` with the implementation of the method checking whether it's less
/// than `0`.
///
/// We used `#` to start a _nested invocation_ of the macro. In it, we use the
/// identifier `int_type_nested` to substitute the 3 unsigned integer types into
/// the body of the nested invocation, which is a substitution group for the
/// outer macro invocation. This therefore produces the three substitution
/// groups that makes the outer macro make the duplicates for the unsigned
/// integers.
///
/// This code is identical to the following, which doesn't use nested
/// invocation:
///
/// ```
/// # use duplicate::duplicate_item;
/// # trait IsNegative {
/// #   fn is_negative(&self) -> bool;
/// # }
/// #[duplicate_item(
///   int_type implementation;
///   [ u8 ]  [ false ];
///   [ u16 ] [ false ];
///   [ u32 ] [ false ];
///   [ i8 ]  [ *self < 0 ]
/// )]
/// impl IsNegative for int_type {
///   fn is_negative(&self) -> bool {
///     implementation
///   }
/// }
/// # assert!(!42u8.is_negative());
/// # assert!(!42u16.is_negative());
/// # assert!(!42u32.is_negative());
/// # assert!(!42i8.is_negative());
/// ```
///
/// Nested invocation is also available for the verbose syntax:
///
/// ```
/// use duplicate::duplicate_item;
/// trait IsNegative {
///   fn is_negative(&self) -> bool;
/// }
///
/// #[duplicate_item(
///   duplicate!{                            // -+
///     [ int_type_nested;[u8];[u16];[u32] ] //  |
///     [                                    //  | Nested invocation producing 3
///       int_type [ int_type_nested ]       //  | substitution groups
///       implementation [ false ]           //  |
///     ]                                    //  |
///   }                                      // -+
///   [                                      // -+
///     int_type [ i8 ]                      //  | Substitution group 4
///     implementation [ *self < 0 ]         //  |
///   ]                                      // -+
/// )]
/// impl IsNegative for int_type {
///   fn is_negative(&self) -> bool {
///     implementation
///   }
/// }
///
/// assert!(!42u8.is_negative());
/// assert!(!42u16.is_negative());
/// assert!(!42u32.is_negative());
/// assert!(!42i8.is_negative());
/// ```
///
/// ## Global Substitution
///
/// ```
/// # struct Some<T1,T2>(T1,T2);
/// # struct Complex<T>(T);
/// # struct Type<T>(T);
/// # struct WeDont<T1,T2,T3>(T1,T2,T3);
/// # struct Want();
/// # struct To();
/// # struct Repeat();
/// # struct Other();
/// # use duplicate::duplicate_item;
/// #[duplicate_item(
///   typ1 [Some<Complex<()>, Type<WeDont<Want, To, Repeat>>>];
///   typ2 [Some<Other, Complex<Type<(To, Repeat)>>>];
///   method     reference(type);
///   [get]      [& type];
///   [get_mut]  [&mut type];
/// )]
/// fn method(
///   arg0: reference([Type<()>]),
///   arg1: typ1,
///   arg2: typ2)
///   -> (reference([typ1]), reference([typ2]))
/// {
///  # /*
///   ...
///  # */
///  # unimplemented!()
/// }
/// ```
///
/// The global substitutions (`typ1` and `typ2`) are substituted in both
/// duplicates of the function. Global substitutions have the same syntax as
/// verbose syntax substitutions, are `;` separated (even from following
/// substitutions groups), must all be defined at the beginning, and aren't
/// usable in the invocation itself but only in the code being duplicated.
#[proc_macro_attribute]
#[cfg_attr(feature = "pretty_errors", proc_macro_error)]
pub fn duplicate_item(attr: TokenStream, item: TokenStream) -> TokenStream
{
	match duplicate_impl(attr, item)
	{
		Ok(result) => result,
		Err(err) => abort(err),
	}
}

/// Substitutes specific identifiers for different code
/// snippets.
///
/// ```
/// # struct Some<T1,T2>(T1,T2);
/// # struct Complex<T>(T);
/// # struct Type<T>(T);
/// # struct WeDont<T1,T2,T3>(T1,T2,T3);
/// # struct Want();
/// # struct To();
/// # struct Repeat();
/// # struct Other();
/// # use duplicate::substitute_item;
/// #[substitute_item(
///   typ1 [Some<Complex<()>, Type<WeDont<Want, To, Repeat>>>];
///   typ2 [Some<Other, Complex<Type<(To, Repeat)>>>];
/// )]
/// fn method(
///   arg1: typ1,
///   arg2: typ2)
///   -> (typ1, typ2)
/// {
///  # /*
///   ...
///  # */
///  # unimplemented!()
/// }
/// ```
///
/// The global substitutions (`typ1` and `typ2`) are substituted in both
/// their occurrences. Global substitutions are `;` separated.
#[proc_macro_attribute]
#[cfg_attr(feature = "pretty_errors", proc_macro_error)]
pub fn substitute_item(attr: TokenStream, item: TokenStream) -> TokenStream
{
	match substitute_impl(attr, item)
	{
		Ok(result) => result,
		Err(err) => abort(err),
	}
}

/// Duplicates the given code and substitutes specific identifiers
/// for different code snippets in each duplicate.
///
/// This is a function-like procedural macro version of [`duplicate_item`].
/// It's functionality is the exact same, and they share the same invocation
/// syntax(es). The only difference is that `duplicate` doesn't only
/// duplicate the following item, but duplicate all code given to it after the
/// invocation block.
///
/// ## Usage
///
/// A call to `duplicate` must start with a `[]` containing the
/// duplication invocation. Everything after that will then be duplicated
/// according to the invocation.
///
/// Given the following `duplicate` call:
/// ```
/// use duplicate::duplicate;
/// # trait IsMax {
/// #   fn is_max(&self) -> bool;
/// # }
///
/// duplicate!{
///   [
///     // Some duplication invocation
/// #     int_type  max_value;
/// #     [ u8 ]    [ 255 ];
/// #     [ u16 ]   [ 65_535 ];
/// #     [ u32 ]   [ 4_294_967_295 ];
///   ]
///   // Some code to duplicate
/// #   impl IsMax for int_type {
/// #     fn is_max(&self) -> bool {
/// #       *self == max_value
/// #     }
/// #   }
/// }
/// # // We use an explicit 'main' function to ensure the previous
/// # // 'duplicate' call doesn't get treated as a statement,
/// # // which illegal before rust 1.45.
/// # fn main() {
/// #   assert!(!42u8.is_max());
/// #   assert!(!42u16.is_max());
/// #   assert!(!42u32.is_max());
/// # }
/// ```
/// It is equivalent to the following invocation using [`duplicate_item`]:
/// ```
/// use duplicate::duplicate_item;
/// # trait IsMax {
/// #   fn is_max(&self) -> bool;
/// # }
///
/// #[duplicate_item(
///   // Some duplication invocation
/// #   int_type  max_value;
/// #   [ u8 ]    [ 255 ];
/// #   [ u16 ]   [ 65_535 ];
/// #   [ u32 ]   [ 4_294_967_295 ];
/// )]
/// // Some code to duplicate
/// # impl IsMax for int_type {
/// #   fn is_max(&self) -> bool {
/// #     *self == max_value
/// #   }
/// # }
/// # assert!(!42u8.is_max());
/// # assert!(!42u16.is_max());
/// # assert!(!42u32.is_max());
/// ```
///
/// For more details on about invocations and features see [`duplicate_item`].
///
/// [`duplicate_item`]: attr.duplicate_item.html
#[proc_macro]
#[cfg_attr(feature = "pretty_errors", proc_macro_error)]
pub fn duplicate(stream: TokenStream) -> TokenStream
{
	inline_macro_impl(stream, duplicate_impl)
}

/// Substitutes specific identifiers for different code
/// snippets.
///
/// This is a function-like procedural macro version of [`substitute_item`].
/// It's functionality is the exact same. The only difference is that
/// `substitute` doesn't only substitute the following item, but all code given
/// to it after the invocation block.
///
/// ```
/// # struct Some<T1,T2>(T1,T2);
/// # struct Complex<T>(T);
/// # struct Type<T>(T);
/// # struct WeDont<T1,T2,T3>(T1,T2,T3);
/// # struct Want();
/// # struct To();
/// # struct Repeat();
/// # struct Other();
/// # use duplicate::substitute;
///
/// substitute!{
///   [
///     typ1 [Some<Complex<()>, Type<WeDont<Want, To, Repeat>>>];
///     typ2 [Some<Other, Complex<Type<(To, Repeat)>>>];
///   ]
///   fn method(
///     arg1: typ1,
///     arg2: typ2)
///     -> (typ1, typ2)
///   {
///    # /*
///     ...
///    # */
///    # unimplemented!()
///   }
/// }
/// ```
///
/// The global substitutions (`typ1` and `typ2`) are substituted in both
/// their occurrences. Global substitutions are `;` separated.
#[proc_macro]
#[cfg_attr(feature = "pretty_errors", proc_macro_error)]
pub fn substitute(stream: TokenStream) -> TokenStream
{
	inline_macro_impl(stream, substitute_impl)
}

/// A result that specified where in the token stream the error occured
/// and is accompanied by a message.
type Result<T> = std::result::Result<T, Error>;

/// Parses an inline macro invocation where the invocation syntax is within
/// initial brackets.
///
/// Extracts the invocation syntax and body to be duplicated/substituted
/// and passes them to the given function.
fn inline_macro_impl(
	stream: TokenStream,
	f: fn(TokenStream, TokenStream) -> Result<TokenStream>,
) -> TokenStream
{
	let empty_globals = SubstitutionGroup::new();
	let mut iter = TokenIter::new(stream, &empty_globals, empty());

	let result = match iter.next_group(Some(Delimiter::Bracket))
	{
		Ok((invocation, _)) =>
		{
			let invocation_body = invocation.to_token_stream();

			f(invocation_body, iter.to_token_stream())
		},
		Err(err) => Err(err.hint("Expected invocation within brackets: [...]")),
	};

	match result
	{
		Ok(result) => result,
		Err(err) => abort(err),
	}
}

/// Implements the duplicate macros.
fn duplicate_impl(attr: TokenStream, item: TokenStream) -> Result<TokenStream>
{
	let dup_def = parse_invocation(attr)?;

	duplicate_and_substitute(
		item,
		&dup_def.global_substitutions,
		dup_def.duplications.iter(),
	)
}

/// Implements the substitute macros
fn substitute_impl(attr: TokenStream, item: TokenStream) -> Result<TokenStream>
{
	duplicate_and_substitute(item, &parse_global_substitutions_only(attr)?, empty())
}

/// Terminates with an error and produces the given message.
///
/// The `pretty_errors` feature can be enabled, the span is shown
/// with the error message.
#[allow(unused_variables)]
fn abort(err: Error) -> !
{
	let (span, msg) = err.extract();
	#[cfg(feature = "pretty_errors")]
	{
		abort!(span, msg);
	}
	#[cfg(not(feature = "pretty_errors"))]
	{
		panic!("{}", msg);
	}
}

#[derive(Debug)]
struct SubstitutionGroup
{
	substitutions: HashMap<String, Substitution>,
	#[cfg(feature = "module_disambiguation")]
	identifier_order: Vec<String>,
}

impl SubstitutionGroup
{
	fn new() -> Self
	{
		Self {
			substitutions: HashMap::new(),
			#[cfg(feature = "module_disambiguation")]
			identifier_order: Vec::new(),
		}
	}

	fn add_substitution(&mut self, ident: Ident, subst: Substitution) -> Result<()>
	{
		if self
			.substitutions
			.insert(ident.to_string(), subst)
			.is_some()
		{
			Err(
				Error::new("Substitution identifier assigned mutiple substitutions")
					.span(ident.span()),
			)
		}
		else
		{
			#[cfg(feature = "module_disambiguation")]
			{
				self.identifier_order.push(ident.to_string());
			}
			Ok(())
		}
	}

	fn substitution_of(&self, ident: &String) -> Option<&Substitution>
	{
		self.substitutions.get(ident)
	}

	fn identifiers(&self) -> impl Iterator<Item = &String>
	{
		self.substitutions.keys()
	}

	fn identifiers_with_args(&self) -> impl Iterator<Item = (&String, usize)>
	{
		self.identifiers()
			.map(move |ident| (ident, self.substitution_of(ident).unwrap().argument_count()))
	}

	#[cfg(feature = "module_disambiguation")]
	fn identifiers_ordered(&self) -> impl Iterator<Item = &String>
	{
		self.identifier_order.iter()
	}
}

/// Defines how duplication should happen.
struct DuplicationDefinition
{
	pub global_substitutions: SubstitutionGroup,
	pub duplications: Vec<SubstitutionGroup>,
}

/// Checks whether item is a module and whether it then needs disambiguation.
///
/// Returns the identifier of the found module (if found) and the substitution
/// identifier that should be used to disambiguate it in each duplicate.
/// Returns none if no disambiguation is needed.
pub(crate) fn disambiguate_module<'a>(
	item: &TokenStream,
	sub_groups: impl Iterator<Item = &'a SubstitutionGroup> + Clone,
) -> Result<Option<(Ident, String)>>
{
	let mut sub_groups = sub_groups.peekable();

	match (sub_groups.peek(), get_module_name(&item))
	{
		(Some(sub), Some(ref module)) if sub.substitution_of(&module.to_string()).is_none() =>
		{
			#[cfg(not(feature = "module_disambiguation"))]
			{
				Err(Error::new(format!(
					"Duplicating the module '{}' without giving each duplicate a unique \
					 name.\nHint: Enable the 'duplicate' crate's 'module_disambiguation' feature \
					 to automatically generate unique module names.",
					module.to_string()
				))
				.span(module.span()))
			}
			#[cfg(feature = "module_disambiguation")]
			{
				let span = module.span();
				Ok(Some((
					module.clone(),
					crate::module_disambiguation::find_simple(sub_groups, span)?,
				)))
			}
		},
		_ => Ok(None),
	}
}

/// Extract the name of the module assuming the given item is a module
/// declaration.
///
/// If not, returns None.
fn get_module_name(item: &TokenStream) -> Option<Ident>
{
	let empty_globals = SubstitutionGroup::new();
	let mut iter = TokenIter::new(item.clone(), &empty_globals, std::iter::empty());

	iter.expect_simple(|t| is_ident(t, Some("mod")), None)
		.ok()?;

	let module = iter.extract_identifier(None).ok()?;
	iter.next_group(Some(Delimiter::Brace)).ok()?;
	Some(module)
}

/// Creates a new group with the given span correctly set as the group's span.
///
/// Use this function instead of creating the group manually, as forgetting
/// to set the span after creating the group could cause problems like leaking
/// this crate's edition into user code or simply result in cryptic error
/// messages.
pub(crate) fn new_group(del: Delimiter, stream: TokenStream, span: Span) -> Group
{
	// We rename 'Group' to not get caught by the 'ensure_no_group_new' test
	use Group as Gr;
	let mut g = Gr::new(del, stream);
	g.set_span(span);
	g
}
