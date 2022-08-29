//! Core command line arguments for all tools

use clap::Arg;

/// Trait to create a new argument
pub trait ArgUtil<'a> {
    /// Create an argument with long argument of the same name
    fn new_argument(name: &'a str) -> Self;
}

/// Implement trait `ArgUtil` for Arg
impl<'a> ArgUtil<'a> for Arg<'a> {
    fn new_argument(name: &'a str) -> Self {
        Arg::new(name).long(name)
    }
}
