const EXPAND_DIR: &'static str = "tests/no_features";
const TESTING_DIR: &'static str = "testing";

#[test]
pub fn test_all_expansions()
{
	let _ = std::fs::remove_dir_all(EXPAND_DIR.to_owned() + "/" + TESTING_DIR);
	std::fs::create_dir_all(EXPAND_DIR.to_owned() + "/" + TESTING_DIR).unwrap();
	copy_from("from");
	if std::path::Path::new(&(EXPAND_DIR.to_owned() + "/expected")).exists()
	{
		copy_from("expected");
	}
	copy_for_both_syntaxes();

	macrotest::expand_without_refresh(EXPAND_DIR.to_owned() + "/" + TESTING_DIR + "/*.rs");
}

fn copy_from(source_dir: &str)
{
	let files = std::fs::read_dir(EXPAND_DIR.to_owned() + "/" + source_dir).unwrap();

	for file in files
	{
		if let Ok(file) = file
		{
			let file_name = file.file_name();
			let file_path = file.path();
			let mut destination = file_path.clone();
			// remove the file name
			destination.pop();
			// remove directory
			destination.pop();
			destination.push(TESTING_DIR);
			destination.push(file_name);
			std::fs::copy(&file_path, &destination).unwrap();
		}
		else
		{
			panic!("Error copying files from: ".to_owned() + source_dir)
		}
	}
}

fn copy_for_both_syntaxes()
{
	let expected_files = std::fs::read_dir(EXPAND_DIR.to_owned() + "/expected_both").unwrap();

	for file in expected_files
	{
		if let Ok(file) = file
		{
			let file_name = file.file_name();
			let file_path = file.path();
			let mut destination_short = file_path.clone();
			// remove the file name
			destination_short.pop();
			// remove the 'expected_both' directory
			destination_short.pop();
			// add the 'testing' directory
			destination_short.push(TESTING_DIR);
			let mut destination_verbose = destination_short.clone();
			destination_short.push("short_".to_owned() + file_name.to_str().unwrap());
			destination_verbose.push("verbose_".to_owned() + file_name.to_str().unwrap());

			std::fs::copy(&file_path, destination_short).unwrap();
			std::fs::copy(&file_path, destination_verbose).unwrap();
		}
		else
		{
			panic!("Error copying expected_both files.")
		}
	}
}
