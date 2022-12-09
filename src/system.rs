//! Module containing utility functions at the operating system level

use crate::warning;
use anyhow::{bail, Result};
use std::{env, fs, process};
use std::{ffi::OsStr, path::Path};

/// Operating system information
mod os {
    /// Keyword to match Windows OS
    pub const WINDOWS: &str = "windows";
    /// Keyword to match Linux OS
    pub const LINUX: &str = "linux";
    /// Keyword to match macOS
    pub const MACOS: &str = "macos";
}

/// Check if the current operating system is a Windows OS
pub fn is_window_os() -> bool {
    std::env::consts::OS.eq(os::WINDOWS)
}

/// Check if the current operating system is Windows
pub fn is_linux_os() -> bool {
    std::env::consts::OS.eq(os::LINUX)
}

/// Find full path of a command from the environment
pub fn path_of_command_from_env(cmd: &str) -> Result<String, String> {
    let finder = match env::consts::OS {
        os::WINDOWS => "where.exe",
        os::LINUX | os::MACOS => "which",
        os => {
            warning!("find_command_path: need to support: {}", os);
            "unknown-finder"
        }
    };

    match process::Command::new(finder).args([cmd]).output() {
        Ok(output) => {
            let cmd_path = String::from_utf8(output.stdout).unwrap_or_default();
            Ok(cmd_path.trim().to_string())
        }

        Err(_) => Err(format!("Command not found: {}", cmd)),
    }
}

/// List all files and sub-directories of a directory
pub fn ls_dir(dir_path: &str) -> Vec<String> {
    match fs::read_dir(dir_path) {
        Ok(paths) => paths
            .into_iter()
            .filter_map(|path| match path {
                Ok(path) => Some(path.path().display().to_string()),
                Err(_) => None,
            })
            .collect(),
        Err(_) => vec![],
    }
}

/// get file extension
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

/// Get the current working directory.
pub fn get_current_directory() -> Result<String> {
    match std::env::current_dir() {
        Ok(path) => match path.to_str() {
            Some(path) => Ok(path.to_owned()),
            None => bail!("Current directory not found!"),
        },
        Err(err) => bail!(err),
    }
}
