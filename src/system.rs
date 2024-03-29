//! Module containing utility functions at the operating system level

use crate::warning;
use anyhow::{bail, Result};
use std::{env, fs, process};

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
