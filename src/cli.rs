//! Core command line arguments for all tools

use crate::global;
use clap::{Arg, ArgMatches, Command};

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

/// Module defining core command line arguments for all tools.
pub mod core_args {
    /// Argument to enable extra printing in debugging mode.
    pub const DEBUG_MODE: &str = "debug-mode";

    /// Argument to enable more printing in deep debugging mode.
    pub const DEEP_DEBUG_MODE: &str = "deep-debug-mode";
}

/// Data structure modelling core command line options for all tools.
#[remain::sorted]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CoreOptions {
    // /// Option to choose a default compiler.
    // pub compiler: Compiler,
    /// Option to enable extra printing in debugging mode.
    pub debug_mode: bool,

    /// Option to enable more extra printing in detailed debugging mode.
    pub deep_debug_mode: bool,
}

/// Data structure handling command line options for assertion checking.
#[remain::sorted]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AssertOptions {
    /// Option to check pointer aliasing assertions.
    pub assert_alias: bool,

    /// Option to check all assertions.
    pub assert_all: bool,

    /// Option to check integer interval assertions.
    pub assert_interval: bool,
}

/// Data structure handling command line options for bug checking.
#[remain::sorted]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct BugOptions {
    /// Option to check all bugs.
    pub all_bugs: bool,

    /// Option to check all integer bugs.
    pub all_integer_bugs: bool,

    /// Option to check all memory bugs.
    pub all_memory_bugs: bool,

    /// Option to check all division-by-zero bugs.
    pub division_by_zero: bool,

    /// Option to check integer-coercion-error bugs.
    pub integer_coercion_error: bool,

    /// Option to check integer-overflow bugs.
    pub integer_overflow: bool,

    /// Option to check integer-underflow bugs.
    pub integer_underflow: bool,

    /// Option to check numeric-truncation-error bugs.
    pub numeric_truncation_error: bool,

    /// Option to check signedness-conversion error bugs related.
    pub signedness_conversion_error: bool,
}

/// Implement methods for `CoreOptions`.
impl<'a> CoreOptions {
    /// Apply the current argument to all core flags.
    pub fn apply_to_core_flags(&self) {
        unsafe {
            global::DEBUG_MODE = self.debug_mode || self.deep_debug_mode;
            global::DEEP_DEBUG_MODE = self.deep_debug_mode;
        }
    }
}

/// Trait to declare core command line arguments of all tools.
pub trait CoreCli {
    /// Configure core command-line arguments of all tools.
    fn configure_core_arguments(self) -> Self;

    /// Configure terminal width of all tools.
    fn configure_terminal_width(self) -> Self;
}

/// Implement the `CoreCli` trait for `Command`.
impl<'a> CoreCli for Command<'a> {
    fn configure_core_arguments(self) -> Self {
        use self::core_args::*;

        self.arg(
            Arg::new_argument(DEBUG_MODE)
                .help("Print debugging information")
                .long("debug")
                .short('d'),
        )
        .arg(
            Arg::new_argument(DEEP_DEBUG_MODE)
                .help("Print deep-debugging information")
                .long("deep-debug")
                .short('D'),
        )
    }

    fn configure_terminal_width(self) -> Self {
        self.term_width(
            if let Some((termsize::Width(w), _)) = termsize::terminal_size() {
                w as usize
            } else {
                120
            },
        )
    }
}

/// Parse core command line arguments shared by all tools.
pub fn parse_core_argument_matches(argms: &ArgMatches) -> CoreOptions {
    use self::core_args::*;

    CoreOptions {
        debug_mode: argms.is_present(DEBUG_MODE),
        deep_debug_mode: argms.is_present(DEEP_DEBUG_MODE),
    }
}
