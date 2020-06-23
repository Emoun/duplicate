//! This crate provides the `duplicate` attribute macro for
//! code duplication with substitution.
//!
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
//! last count.) We can use the `duplicate` attribute to avoid repeating
//! ourselves:
//!
//! ```
//! # trait IsMax {fn is_max(&self) -> bool;}
//! use duplicate::duplicate;
//! #[duplicate(
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
//! is valid. Additionally, any "bracket" type is valid; we could have used `()`
//! or `{}` anywhere `[]` is used in these examples.
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
//! # use duplicate::duplicate;
//! # struct VecWrap<T>(Vec<T>);
//! impl<T> VecWrap<T> {
//!   #[duplicate(
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
//! In a `duplicate` invocation, if a substitution identifier is followed by
//! brackets containing a list of parameters, they can be used in the
//! substitution. In this example, the `reference` identifier takes 1 parameter
//! named `type`, which is used in the substitutions to create either a shared
//! reference to the type or a mutable one. When using the `reference` in the
//! method declaration, we give it different types as arguments to construct
//! either shared or mutable references.
//! E.g. `reference([Self])` becomes `&Self` in the first duplicate and `&mut
//! Self` in the second. An argument can be any code snippet inside brackets.
//!
//! A substitution identifier can take any number of parameters.
//! We can use this if we need to also provide the references with a lifetime:
//!
//! ```
//! # use duplicate::duplicate;
//! # struct VecWrap<T>(Vec<T>);
//! impl<T> VecWrap<T> {
//!   #[duplicate(
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
//! `duplicate` itself.
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
//! Notice how the code repetition is split over 2 axes: 1) They all implement
//! the same trait 2) the method implementations of the first 3 are identical to
//! each other but different to the next 3, which are also mutually identical.
//! To implement this using only the syntax we have already seen, we could do
//! something like this:
//! ```
//! # trait IsNegative { fn is_negative(&self) -> bool;}
//! # use duplicate::duplicate;
//! #[duplicate(
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
//! However, ironically, we here had to repeat ourselves in the macro invocation
//! instead of the code: we needed to repeat the implementations `[ false ]` and
//! `[ *self < 0 ]` three times each. We can utilize _nested invocation_ to
//! remove the last bit of repetition:
//!
//! ```
//! # trait IsNegative { fn is_negative(&self) -> bool;}
//! # use duplicate::duplicate;
//! #[duplicate(
//!   int_type implementation;
//!   #[
//!     int_type_nested; [u8]; [u16]; [u32]
//!   ][
//!     [ int_type_nested ] [ false ];
//!   ]
//!   #[
//!     int_type_nested; [i8]; [i16]; [i32]
//!   ][
//!     [ int_type_nested ] [ *self < 0 ];
//!   ]
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
//! We use `#` to invoke the macro inside itself, producing duplicates
//! of the code inside the following `[]`, `{}`, or `()`.
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
//! normal substution groups. For example, say we want to implement `IsNegative`
//! for `i8`, but don't want the same for `i16` and `i32`. We could do the
//! following:
//!
//! ```
//! # trait IsNegative { fn is_negative(&self) -> bool;}
//! # use duplicate::duplicate;
//! #[duplicate(
//!   int_type implementation;
//!   #[                                     // -+
//!     int_type_nested; [u8]; [u16]; [u32]  //  | Nested invocation producing 3
//!   ][                                     //  | substitution groups
//!     [int_type_nested ] [ false ];        //  |
//!   ]                                      // -+
//!   [ i8 ] [ *self < 0 ]                   // -- Substitution group 4
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
//! Note that nested invocation is only allowed after the initial list of
//! substitution identifiers. You also cannot use it between individual
//! subtitutions in a group, only between whole substitution groups.
//! Lastly, remember that substitution groups must be seperated by `;`, which
//! means the nested invocation must produce these semi-colons explicitly and
//! correctly.
//!
//! ## Verbose Syntax
//!
//! The syntax used in the previous examples is the _short syntax_.
//! `duplicate` also accepts a _verbose syntax_ that is less concise, but more
//! readable in some circumstances. Using the verbose syntax, the very first
//! example above looks like this:
//!
//! ```
//! # trait IsMax {fn is_max(&self) -> bool;}
//! use duplicate::duplicate;
//! #[duplicate(
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
//! In the verbose syntax, a substitution group is put inside brackets and
//! includes a list of substitution identifiers followed by their substitutions.
//! No `;`s are needed. Here is an annotated version of the same code:
//!
//! ```
//! # trait IsMax {fn is_max(&self) -> bool;}
//! # use duplicate::duplicate;
//! #[duplicate(
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
//! The verbose syntax does not currently support parameterized substitution.
//! It is planned to be added to this syntax in a future update.
//!
//! The verbose syntax does offer nested invocation. The syntax is exactly the
//! same, but since there is no initial substitution identifier list, nested
//! calls can be used anywhere (though still not inside substitution groups.)
//! The previous `IsNegative` nested invocation example can be written as
//! follows:
//!
//! ```
//! # trait IsNegative { fn is_negative(&self) -> bool;}
//! # use duplicate::duplicate;
//! #[duplicate(
//!   #[
//!     int_type_nested; [u8]; [u16]; [u32]
//!   ][
//!     [
//!       int_type [ int_type_nested ]
//!       implementation [ false ]
//!     ]
//!   ]
//!   #[
//!     int_type_nested; [i8]; [i16]; [i32]
//!   ][
//!     [
//!       int_type [ int_type_nested ]
//!       implementation [ *self < 0 ]
//!     ]
//!   ]
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
//! # Disclaimer
//!
//! This crate does not try to justify or condone the usage of code duplication
//! instead of proper abstractions.
//! This macro should only be used where it is not possible to reduce code
//! duplication through other means, or where it simply is not worth it.
//!
//! As an example, libraries that have two or more structs/traits with similar
//! APIs might use this macro to test them without having to copy-paste test
//! cases and manually make the needed edits.
use proc_macro::{Span, TokenStream};
use proc_macro_error::*;

mod parse;
mod parse_utils;
mod substitute;
// Tests the crate readme file's Rust examples.
mod crate_readme_test;

use parse::*;
use substitute::*;

/// Duplicates the item it is applied to and substitutes specific identifiers
/// for different code snippets in each duplicate.
///
/// # Short Syntax
/// ```
/// use duplicate::duplicate;
/// trait IsMax {
///   fn is_max(&self) -> bool;
/// }
///
/// #[duplicate(
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
/// The substitutions must be enclosed in `[]`, `{}`, or `()`, but are otherwise
/// free.
///
/// # Verbose Syntax
///
/// ```
/// use duplicate::duplicate;
/// trait IsMax {
///   fn is_max(&self) -> bool;
/// }
///
/// #[duplicate(
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
/// `[]`, `{}`, or `()`. A substitution group is a set of identifiers and
/// substitution pairs, like in the short syntax, but there can only be one
/// substitution per identifier. All substitution groups must have the same
/// identifiers, however their order is unimportant, as can be seen from the
/// last substitution group above, where `max_value` comes before `int_type`.
///
/// # Parameterized Substitutoin
///
/// _Only available for the short syntax._
///
/// ```
/// use duplicate::duplicate;
/// struct VecWrap<T>(Vec<T>);
///
/// impl<T> VecWrap<T> {
///   #[duplicate(
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
/// If an identifier is followed by brackets (in both its declaration and its
/// use), a set of parameters can be provided in the bracket to customize the
/// subtituion for each use.
/// In the declaration a list of identifiers is given, which can be used in its
/// substitutions. When using the identifier, argument code snippets must be
/// given in a comma separated list, with each argument being inclosed in
/// brackets (`()`, `[]`, or `{}`).
///
/// # Nested Invocation
/// ```
/// use duplicate::duplicate;
/// trait IsNegative {
///   fn is_negative(&self) -> bool;
/// }
///
/// #[duplicate(
///   int_type implementation;
///   #[                                  // -+
///     int_type_nested;[u8];[u16];[u32]  //  | Nested invocation producing 3
///   ][                                  //  | substitution groups
///     [ int_type_nested ] [ false ];    //  |
///   ]                                   // -+
///   [ i8 ] [ *self < 0 ]                // -- Substitution group 4
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
/// # use duplicate::duplicate;
/// # trait IsNegative {
/// #   fn is_negative(&self) -> bool;
/// # }
/// #[duplicate(
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
/// use duplicate::duplicate;
/// trait IsNegative {
///   fn is_negative(&self) -> bool;
/// }
///
/// #[duplicate(
///   #[                                  // -+
///     int_type_nested;[u8];[u16];[u32]  //  |
///   ][                                  //  |
///     [                                 //  | Nested invocation producing 3
///       int_type [ int_type_nested ]    //  | substitution groups
///       implementation [ false ]        //  |
///     ]                                 //  |
///   ]                                   // -+
///   [                                   // -+
///     int_type [ i8 ]                   //  | Substitution group 4
///     implementation [ *self < 0 ]      //  |
///   ]                                   // -+
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
#[proc_macro_attribute]
#[proc_macro_error]
pub fn duplicate(attr: TokenStream, item: TokenStream) -> TokenStream
{
	match duplicate_impl(attr, item)
	{
		Ok(result) => result,
		Err(err) => abort!(err.0, err.1),
	}
}

/// Implements the macro.
///
/// `allow_short`: If true, accepts short syntax
fn duplicate_impl(attr: TokenStream, item: TokenStream) -> Result<TokenStream, (Span, String)>
{
	let subs = parse_attr(attr, Span::call_site())?;
	let result = substitute(item, subs);
	Ok(result)
}
