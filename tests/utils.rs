use std::{
	ffi::OsString,
	fs::{DirEntry, File},
	io::{BufRead, BufReader, Write},
	path::Path,
};

/// Whether the `pretty_errors` feature is enabled.
pub const FEATURE_PRETTY_ERRORS: bool = cfg!(feature = "pretty_errors");
/// Whether the `module_disambiguation` feature is enabled.
pub const FEATURE_MODULE_DISAMBIGUATION: bool = cfg!(feature = "module_disambiguation");
/// The number of enabled features.
pub const NR_FEATURES: usize =
	0 + FEATURE_PRETTY_ERRORS as usize + FEATURE_MODULE_DISAMBIGUATION as usize;
/// A list of the enabled features.
const FEATURES: [&'static str; NR_FEATURES] = get_features();

/// Returns a list of enabled features.
const fn get_features() -> [&'static str; NR_FEATURES]
{
	#[allow(unused_mut)]
	let mut features: [&'static str; NR_FEATURES] = [""; NR_FEATURES];
	#[cfg(feature = "pretty_errors")]
	{
		features[0] = "pretty_errors";
	}
	#[cfg(feature = "module_disambiguation")]
	{
		features[FEATURE_PRETTY_ERRORS as usize] = "module_disambiguation";
	}
	features
}

/// Manages the setting up and running of expansion tests using macrotest
///
/// Expansion test live in a home directory. This directory has a single
/// testing sub-directory that is used during the test. Temporary testing
/// files are put in the testing directory before each test but not removed
/// after. (They may be deleted before each test, though)
///
/// The tester is configured to generate files in the testing directory from
/// files in source directories (sub-directories of the home).
/// Various rules can be configured, e.g. a simple copy of files, or duplicating
/// the source files a number of times in the testing directory with various
/// names.
pub struct ExpansionTester<'a>
{
	/// The home directory for the tests
	dir: &'a str,
	/// The subdirectory (of the home) where test files may be put
	testing_dir: &'a str,
	/// Source sub-directory, and how it's files should be treated before
	/// testing
	source_dirs: Vec<(&'a str, Vec<Box<dyn Fn(&DirEntry, &dyn AsRef<Path>)>>)>,
}

impl<'a> ExpansionTester<'a>
{
	/// Construct a new tester with a home directory and a testing subdirectory.
	pub fn new(home_dir: &'a str, testing_dir: &'a str) -> Self
	{
		Self {
			dir: home_dir,
			testing_dir,
			source_dirs: Vec::new(),
		}
	}

	/// Add a source directory under the home directory,
	/// with a list of actions that produce files in the testing directory
	/// based on each file in the source directory.
	pub fn add_source_dir(
		&mut self,
		dir: &'a str,
		actions: Vec<Box<dyn Fn(&DirEntry, &dyn AsRef<Path>)>>,
	)
	{
		self.source_dirs.push((dir, actions));
	}

	/// Executes the tests including first setting up the testing directory.
	pub fn execute_tests(&self)
	{
		// Remove old test files
		let testing_dir = self.dir.to_owned() + "/" + self.testing_dir;
		let _ = std::fs::remove_dir_all(&testing_dir);

		// Recreate testing dir
		std::fs::create_dir_all(&testing_dir).unwrap();

		// For each source dir, execute action of each file
		for (source_dir, actions) in self.source_dirs.iter()
		{
			let source_dir_path = self.dir.to_owned() + "/" + source_dir;
			if let Ok(files) = std::fs::read_dir(&source_dir_path)
			{
				for file in files
				{
					if let Ok(file) = file
					{
						for action in actions.iter()
						{
							action(&file, &testing_dir);
						}
					}
					else
					{
						panic!("Error accessing source file: {:?}", file)
					}
				}
			}
		}

		// Prepare feature list for expansion testing
		let mut args: Vec<&str> = Vec::new();
		let mut features = String::new();
		if NR_FEATURES > 0
		{
			args.push("--features");
			for f in FEATURES.iter()
			{
				features.push_str(f);
				features.push(',');
			}
			args.push(features.as_str());
		}

		macrotest::expand_without_refresh_args(testing_dir + "/*.rs", args.as_slice());
	}

	/// Generates an action that copies the file given to the testing
	/// directory with the given prefix added to its name.
	pub fn copy_with_prefix(prefix: &str) -> Box<dyn Fn(&DirEntry, &dyn AsRef<Path>)>
	{
		let prefix = OsString::from(prefix);
		Box::new(move |file, destination| {
			let mut destination_file = destination.as_ref().to_path_buf();
			let mut file_name = prefix.clone();
			file_name.push(file.file_name());
			destination_file.push(file_name);
			std::fs::copy(&file.path(), &destination_file).unwrap();
		})
	}

	/// Generates an action that simply copies the file given to the testing
	/// directory.
	pub fn copy() -> Box<dyn Fn(&DirEntry, &dyn AsRef<Path>)>
	{
		Self::copy_with_prefix("")
	}

	/// Generates an action that creates two versions of the given file in the
	/// testing directory. The source file must use the 'duplicate' attribute
	/// macro, where:
	/// - The invocation must starts with `#[duplicate::duplicate_item(` on a
	///   the first line
	/// (with nothing else). Notice that you must not import the attribute but
	/// use its full path.
	/// - Then the body of the invocation. Both syntaxes are allowed.
	/// - Then the `)]` on its own line, followed immediately by
	///   `//duplicate_end`.
	/// I.e. `)]//duplicate_end`
	/// - Then the item to be duplicated, followed on the next line by
	///   `//item_end` on
	/// its own.
	///
	/// This action will then generate 2 versions of this file. The first is
	/// almost identical the original, but the second will change the invocation
	/// to instead use `duplicate`. It uses the exact rules specified
	/// above to correctly change the code, so any small deviation from the
	/// above rules might result in an error. The name of the first version is
	/// the same as the original and the second version is prefixed with
	/// 'inline_'
	///
	/// ### Example
	/// Original file (`test.rs`):
	/// ```
	/// #[duplicate::duplicate_item(
	///   name;
	///   [SomeName];
	/// )]//duplicate_end
	/// pub struct name();
	/// //item_end
	/// ```
	/// First version (`test.expanded.rs`):
	/// ```
	/// #[duplicate::duplicate_item(
	///   name;
	///   [SomeName];
	/// )]
	/// pub struct name();
	/// ```
	/// Second version (`inline_test.expanded.rs`):
	/// ```
	/// duplicate::duplicate{
	///   [
	///     name;
	///     [SomeName];
	///   ]
	///   pub struct name();
	/// }
	/// ```
	pub fn duplicate_for_inline() -> Box<dyn Fn(&DirEntry, &dyn AsRef<Path>)>
	{
		Box::new(|file, destination| {
			let mut inline_file_name = OsString::from("inline_");
			inline_file_name.push(file.file_name());

			let mut dest_file_path = destination.as_ref().to_path_buf();
			let mut dest_inline_file_path = destination.as_ref().to_path_buf();

			dest_file_path.push(file.file_name());
			dest_inline_file_path.push(inline_file_name);

			let mut dest_file = File::create(dest_file_path).unwrap();
			let mut dest_inline_file = File::create(dest_inline_file_path).unwrap();

			for line in BufReader::new(File::open(file.path()).unwrap()).lines()
			{
				let line = line.unwrap();
				let line = line.trim();

				match line
				{
					"#[duplicate::duplicate_item(" =>
					{
						dest_file
							.write_all("#[duplicate::duplicate_item(".as_bytes())
							.unwrap();
						dest_inline_file
							.write_all("duplicate::duplicate!{\n[".as_bytes())
							.unwrap();
					},
					")]//duplicate_end" =>
					{
						dest_file.write_all(")]".as_bytes()).unwrap();
						dest_inline_file.write_all("]".as_bytes()).unwrap();
					},
					"//item_end" =>
					{
						dest_inline_file.write_all("}".as_bytes()).unwrap();
					},
					_ =>
					{
						dest_file.write_all(line.as_bytes()).unwrap();
						dest_inline_file.write_all(line.as_bytes()).unwrap();
					},
				}
				dest_file.write_all("\n".as_bytes()).unwrap();
				dest_inline_file.write_all("\n".as_bytes()).unwrap();
			}
		})
	}

	/// Sets up and runs tests in a specific directory using our standard test
	/// setup.
	pub fn run_default_test_setup(home_dir: &str, test_subdir: &str)
	{
		let mut test = ExpansionTester::new(home_dir, test_subdir);
		test.add_source_dir("from", vec![ExpansionTester::duplicate_for_inline()]);
		test.add_source_dir(
			"expected",
			vec![
				ExpansionTester::copy(),
				ExpansionTester::copy_with_prefix("inline_"),
			],
		);
		test.add_source_dir(
			"expected_both",
			vec![
				ExpansionTester::copy_with_prefix("inline_short_"),
				ExpansionTester::copy_with_prefix("inline_verbose_"),
				ExpansionTester::copy_with_prefix("short_"),
				ExpansionTester::copy_with_prefix("verbose_"),
			],
		);
		test.execute_tests();
	}
}
