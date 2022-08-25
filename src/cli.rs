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
    /// Argument to pass specific options to Clang.
    pub const CLANG_OPTIONS: &str = "clang-options";

    // /// Argument to choose a default compiler.
    // pub const COMPILER: &str = "compiler";

    /// Argument to enable extra printing in debugging mode.
    pub const DEBUG_MODE: &str = "debug-mode";

    /// Argument to enable more printing in deep debugging mode.
    pub const DEEP_DEBUG_MODE: &str = "deep-debug-mode";

}

/// Module defining command line arguments for assertion checking.
pub mod assert_args {
    /// Argument to enable checking all assertions.
    pub const ASSERT_ALL: &str = "assert-all";

    /// Argument to enable checking integer interval assertions.
    pub const ASSERT_INTERVAL: &str = "assert-interval";

    /// Argument to enable checking pointer aliasing assertions.
    pub const ASSERT_ALIAS: &str = "assert-alias";
}

/// Module defining command line arguments for bug checking.
pub mod bug_args {
    /// Argument to enable checking all integer bugs
    pub const INTEGER_ALL: &str = "bug-integer-all";

    /// Argument to enable checking division-by-zero bugs
    pub const DIVISION_BY_ZERO: &str = "division-by-zero";

    /// Argument to enable checking integer-overflow bugs
    pub const INTEGER_OVERFLOW: &str = "bug-integer-overflow";

    /// Argument to enable checking integer-underflow bugs
    pub const INTEGER_UNDERFLOW: &str = "bug-integer-underflow";

    /// Argument to enable checking integer-coercion-error bugs
    pub const INTEGER_COERCION_ERR: &str = "bug-integer-coercion-error";

    /// Argument to enable checking all memory bugs
    pub const MEMORY_ALL: &str = "bug-memory-all";

    /// Argument to enable checking numeric-truncation-error bugs
    pub const NUMERIC_TRUNCATION_ERR: &str = "bug-numeric-truncation-error";

    /// Argument to enable checking all integer bugs
    pub const BUG_ALL: &str = "bug-all";

    /// Argument to enable checking type-conversion bugs
    pub const TYPE_CONVERSION_ERR: &str = "bug-type-conversion";
}

/// Data structure modelling core command line options for all tools.
#[remain::sorted]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CoreOptions<'a> {
    /// Option to pass specific options to Clang.
    pub clang_options: Vec<&'a str>,

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

/// Implement methods for `AssertOptions`.
impl AssertOptions {
    /// Check if aliasing assertion checking is enabled.
    pub fn need_to_check_aliasing(&self) -> bool {
        self.assert_all || self.assert_alias
    }

    /// Check if interval assertion checking is enabled.
    pub fn need_to_check_interval(&self) -> bool {
        self.assert_all || self.assert_alias
    }

    /// Check if all assertion checking is enabled.
    pub fn need_to_check_assertions(&self) -> bool {
        self.need_to_check_aliasing() || self.need_to_check_interval()
    }
}

/// Implement methods for `CoreOptions`.
impl<'a> CoreOptions<'a> {
    /// Apply the current argument to all core flags.
    pub fn apply_to_core_flags(&self) {
        unsafe {
            global::DEBUG_MODE = self.debug_mode || self.deep_debug_mode;
            global::DEEP_DEBUG_MODE = self.deep_debug_mode;
        }
    }
}

/// Implement methods for `BugOptions`.
impl BugOptions {
    /// Check if the feature to find integer bugs is enabled.
    pub fn need_to_check_integer_bugs(&self) -> bool {
        self.all_bugs
            || self.all_integer_bugs
            || self.division_by_zero
            || self.integer_overflow
            || self.integer_underflow
            || self.integer_coercion_error
            || self.numeric_truncation_error
            || self.signedness_conversion_error
    }

    /// Check if the feature to find memory bugs is enabled.
    pub fn need_to_check_memory_bugs(&self) -> bool {
        self.all_bugs || self.all_memory_bugs
    }

    /// Check if the feature to find all bugs is enabled.
    pub fn need_to_check_bugs(&self) -> bool {
        self.need_to_check_memory_bugs() || self.need_to_check_integer_bugs()
    }
}

/// Trait to declare core command line arguments of all tools.
pub trait CoreCli {
    /// Configure core command-line arguments of all tools.
    fn configure_core_arguments(self) -> Self;

    /// Configure command-line arguments for assertion checking.
    fn configure_assert_arguments(self) -> Self;

    /// Configure command-line arguments for bug checking.
    fn configure_bug_arguments(self) -> Self;

    /// Configure terminal width of all tools.
    fn configure_terminal_width(self) -> Self;
}

/// Implement the `CoreCli` trait for `Command`.
impl<'a> CoreCli for Command<'a> {
    fn configure_core_arguments(self) -> Self {
        use self::core_args::*;

        self.arg(
            Arg::new_argument(CLANG_OPTIONS)
                .help("User-provided options for Clang")
                .takes_value(true)
                .allow_hyphen_values(true)
                .allow_invalid_utf8(true)
                .display_order(2),
        )
        // .arg(
        //     Arg::new_argument(COMPILER)
        //         .help("Choose compiler clang/solang/solc")
        //         .allow_invalid_utf8(true)
        //         .takes_value(true)
        //         .multiple_values(false)
        //         .display_order(2),
        // )
        .arg(
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

    fn configure_assert_arguments(self) -> Self {
        use self::assert_args::*;

        self.arg(
            Arg::new_argument(ASSERT_ALL)
                .help("Enable checking all assertions")
                .display_order(2),
        )
        .arg(
            Arg::new_argument(ASSERT_INTERVAL)
                .help("Enable checking all integer interval assertions")
                .display_order(2),
        )
        .arg(
            Arg::new_argument(ASSERT_ALIAS)
                .help("Enable checking pointer aliasing assertions")
                .display_order(2),
        )
    }

    fn configure_bug_arguments(self) -> Self {
        use self::bug_args::*;

        self.arg(
            Arg::new_argument(INTEGER_ALL)
                .help("Enable checking all integer bugs")
                .display_order(2),
        )
        .arg(
            Arg::new_argument(DIVISION_BY_ZERO)
                .help("Enable checking division-by-zero bugs")
                .display_order(2),
        )
        .arg(
            Arg::new_argument(INTEGER_OVERFLOW)
                .help("Enable checking integer-overflow bugs")
                .display_order(2),
        )
        .arg(
            Arg::new_argument(INTEGER_UNDERFLOW)
                .help("Enable checking integer-underflow bugs")
                .display_order(2),
        )
        .arg(
            Arg::new_argument(INTEGER_COERCION_ERR)
                .help("Enable checking integer-coercion-error bugs")
                .display_order(2),
        )
        .arg(
            Arg::new_argument(TYPE_CONVERSION_ERR)
                .help("Enable checking type-conversion-error  bugs")
                .display_order(2),
        )
        .arg(
            Arg::new_argument(NUMERIC_TRUNCATION_ERR)
                .help("Enable checking numeric-truncation-error bugs")
                .display_order(2),
        )
        .arg(
            Arg::new_argument(MEMORY_ALL)
                .help("Enable checking all memory bugs")
                .display_order(2),
        )
        .arg(
            Arg::new_argument(BUG_ALL)
                .help("Enable checking all bugs")
                .display_order(2),
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

    let clang_user_options = match argms.values_of_os(CLANG_OPTIONS) {
        None => vec![],
        Some(ss) => ss.into_iter().filter_map(|v| v.to_str()).collect(),
    };

    // let compiler = match argms.values_of_os(COMPILER) {
    //     None => Compiler::Unknown,
    //     Some(compiler_info) => {
    //         let compilers = compiler_info
    //             .into_iter()
    //             .filter_map(|v| v.to_str())
    //             .collect::<Vec<&str>>();
    //         if compilers.len() != 1 {
    //             panic!("Parsing CLI: expect 1 user-provided compiler")
    //         } else if compilers[0].eq(tool::CLANG) {
    //             Compiler::Clang
    //         } else if compilers[0].eq(tool::SOLANG) {
    //             Compiler::Solang
    //         } else if compilers[0].eq(tool::SOLC) {
    //             Compiler::Solc
    //         } else {
    //             Compiler::Unknown
    //         }
    //     }
    // };

    CoreOptions {
        // compiler,
        clang_options: clang_user_options,
        debug_mode: argms.is_present(DEBUG_MODE),
        deep_debug_mode: argms.is_present(DEEP_DEBUG_MODE),
    }
}

/// Parse command line arguments for assertion checking, shared by some tools.
pub fn parse_assert_argument_matches(argms: &ArgMatches) -> AssertOptions {
    use self::assert_args::*;
    AssertOptions {
        assert_all: argms.is_present(ASSERT_ALL),
        assert_interval: argms.is_present(ASSERT_INTERVAL),
        assert_alias: argms.is_present(ASSERT_ALIAS),
    }
}

/// Parse command line arguments for bug checking, shared by some tools.
pub fn parse_bug_argument_matches(argms: &ArgMatches) -> BugOptions {
    use self::bug_args::*;
    let all_bugs = argms.is_present(BUG_ALL);

    // Integer bugs
    let all_integer_bugs = argms.is_present(INTEGER_ALL) || all_bugs;
    let integer_overflow =
        argms.is_present(INTEGER_OVERFLOW) || all_integer_bugs;
    let integer_underflow =
        argms.is_present(INTEGER_UNDERFLOW) || all_integer_bugs;
    let numeric_truncation_error =
        argms.is_present(NUMERIC_TRUNCATION_ERR) || all_integer_bugs;
    let division_by_zero =
        argms.is_present(DIVISION_BY_ZERO) || all_integer_bugs;
    let integer_coercion_error =
        argms.is_present(INTEGER_COERCION_ERR) || all_integer_bugs;
    let type_conversion_error =
        argms.is_present(TYPE_CONVERSION_ERR) || all_integer_bugs;

    // Memory bugs
    let all_memory_bugs = argms.is_present(MEMORY_ALL);

    BugOptions {
        division_by_zero,
        integer_coercion_error,
        numeric_truncation_error,
        integer_overflow,
        signedness_conversion_error: type_conversion_error,
        integer_underflow,
        all_integer_bugs,
        all_memory_bugs,
        all_bugs,
    }
}
