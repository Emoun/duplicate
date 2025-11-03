/// Returns the directory of the calling file as a &str
macro_rules! file_dir {
	() => {{
		let mut buf = std::path::PathBuf::from(file!());
		buf.pop();
		buf.to_str().unwrap().to_string().as_str()
	}};
}

#[cfg(feature = "default")]
mod default_features;
mod errors;
#[cfg(feature = "module_disambiguation")]
mod module_disambiguation;
mod no_features;
mod utils;
