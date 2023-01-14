//! Module containing utility functions to handle files.

use std::{ffi::OsStr, fs::File, io::Write, path::Path};

use anyhow::{bail, Result};

/// Function to save a string to a temporary file of a given name.
///
/// Return the output file path.
pub fn save_to_temporary_file(content: &str, filename: &str) -> Result<String> {
    let output_dir_path = match tempfile::tempdir() {
        Ok(dir) => dir.into_path(),
        Err(err) => bail!(err),
    };
    let output_file_path = output_dir_path.join(filename);
    let mut output_file = File::create(&output_file_path)?;
    match output_file.write_all(content.as_bytes()) {
        Ok(_) => match output_file_path.to_str() {
            Some(path) => Ok(path.to_string()),
            None => bail!("Output file path not found!"),
        },
        Err(err) => bail!(err),
    }
}

/// Get file extension of a file name.
pub fn get_file_ext(filename: &str) -> Option<&str> {
    Path::new(filename).extension().and_then(OsStr::to_str)
}

/// Get the parent directory of a file.
///
/// Return `None` if the parent directory is not found or empty.
pub fn get_parent_directory(filename: &str) -> Option<String> {
    let file_path = Path::new(&filename);

    let parent_dir = match file_path.parent() {
        Some(path) => path.to_str().unwrap_or(""),
        None => "",
    };

    match parent_dir.is_empty() {
        true => None,
        false => Some(parent_dir.to_owned()),
    }
}
