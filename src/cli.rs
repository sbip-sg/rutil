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

    /// Argument to disable code instrument.
    pub const DISABLE_INSTRUMENT: &str = "disable-instrument";

    /// Argument to disable code normalize
    pub const DISABLE_NORMALIZE: &str = "disable-normalize";

    /// Argument to disable code optimize.
    pub const DISABLE_OPTIMIZE: &str = "disable-optimize";

    /// Argument to disable printing.
    pub const DISABLE_PRINTING: &str = "disable-printing";

    /// Argument to enable generating YUL statistics.
    pub const GENERATE_YUL_STATISTICS: &str = "generate-yul-statistics";

    /// Argument to pass the directory path including supporting libraries.
    pub const INCLUDE_DIR: &str = "include-dir";

    /// Argument to pass the file path of a library files.
    pub const INCLUDE_FILE: &str = "include-file";

    /// Argument to instrument bug annotation.
    pub const INSTRUMENT_BUG_ANNOTS: &str = "instrument-bug-annots";

    /// Argument to print the compiled program to `stdout`.
    pub const PRINT_COMPILED_PROG: &str = "print-compiled-program";

    /// Argument to print the final program, which is obtained after all
    /// compilation and normalize steps.
    pub const PRINT_FINAL_PROG: &str = "print-final-program";

    /// Argument to print the instrumented program to `stdout`.
    pub const PRINT_INSTRUMENTED_PROG: &str = "print-instrumented-program";

    /// Argument to print the main program to `stdout`.
    pub const PRINT_MAIN_PROG: &str = "print-main-program";

    /// Argument to print the normalized program to `stdout`.
    pub const PRINT_NORMALIZED_PROG: &str = "print-normalized-program";

    /// Argument to print the optimized program to `stdout`.
    pub const PRINT_OPTIMIZED_PROG: &str = "print-optimized-program";

    /// Argument to print the sparse program to `stdout`.
    pub const PRINT_SPARSE_PROG: &str = "print-sparse-program";

    /// Argument to pass specific options to Rustc
    pub const RUSTC_OPTIONS: &str = "rustc-options";

    /// Argument to pass specific options to Solang
    pub const SOLANG_OPTIONS: &str = "solang-options";

    /// Argument to pass specific options to Solc
    pub const SOLC_OPTIONS: &str = "solc-options";
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

    /// Option to disable instrumenting code.
    pub disable_instrumentation: bool,

    /// Option to disable normalize.
    pub disable_normalization: bool,

    /// Option to disable optimize.
    pub disable_optimization: bool,

    /// Option to disable printing.
    pub disable_printing: bool,

    /// Option to enable generating statistics of YUL IR.
    pub generate_yul_statistics: bool,

    /// Option to capture directory paths containing supporting libraries.
    pub include_dirs: Vec<&'a str>,

    /// Option to capture file paths of supporting libraries.
    pub include_files: Vec<&'a str>,

    /// Option to enable code instrument.
    pub instrument_code: bool,

    /// Option to print the compiled program to `stdout`.
    pub print_compiled_prog: bool,

    /// Option to print the final program to `stdout`.
    pub print_final_prog: bool,

    /// Option to print instrumented program to `stdout`.
    pub print_instrumented_prog: bool,

    /// Option to print the main program to `stdout`.
    pub print_main_prog: bool,

    /// Option to print normalized program to `stdout`.
    pub print_normalized_prog: bool,

    /// Option to print optimized program to `stdout`.
    pub print_optimized_prog: bool,

    /// Option to print the sparse program to `stdout`.
    pub print_sparse_prog: bool,

    /// Option to pass specific options to Rustc.
    pub rustc_options: Vec<&'a str>,

    /// Option to pass specific options to Solang.
    pub solang_options: Vec<&'a str>,

    /// Option to pass specific options to Solc.
    pub solc_options: Vec<&'a str>,
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
impl<'a> CoreOptions<'a> {
    /// Apply the current argument to all core flags.
    pub fn apply_to_core_flags(&self) {
        unsafe {
            global::DEBUG_MODE = self.debug_mode || self.deep_debug_mode;
            global::DEEP_DEBUG_MODE = self.deep_debug_mode;
            global::DISABLE_PRINTING = self.disable_printing;
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
            Arg::new_argument(CLANG_OPTIONS)
                .help("User-provided options for Clang")
                .takes_value(true)
                .allow_hyphen_values(true)
                .allow_invalid_utf8(true)
                .display_order(2),
        )
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
        .arg(
            Arg::new_argument(DISABLE_INSTRUMENT)
                .help("Disable the code instrument pass")
                .display_order(2),
        )
        .arg(
            Arg::new_argument(DISABLE_NORMALIZE)
                .help("Disable the code normalize pass")
                .display_order(2),
        )
        .arg(
            Arg::new_argument(DISABLE_OPTIMIZE)
                .help("Disable the code optimize pass")
                .display_order(2),
        )
        .arg(
            Arg::new_argument(DISABLE_PRINTING)
                .help("Disable generic information printing")
                .display_order(2),
        )
        .arg(
            Arg::new_argument(GENERATE_YUL_STATISTICS)
                .help("Enable generating YUL statistics")
                .display_order(2),
        )
        .arg(
            Arg::new_argument(INCLUDE_DIR)
                .help("Path of a directory containing supporting libraries")
                .takes_value(true)
                .multiple_occurrences(true)
                .allow_invalid_utf8(true)
                .display_order(2)
                .short('I'),
        )
        .arg(
            Arg::new_argument(INCLUDE_FILE)
                .help("Path of a file contating supporting libraries")
                .takes_value(true)
                .multiple_occurrences(true)
                .allow_invalid_utf8(true)
                .display_order(2)
                .short('i'),
        )
        .arg(
            Arg::new_argument(INSTRUMENT_BUG_ANNOTS)
                .help("Instrument bug annotations into bitcode")
                .visible_alias("iba")
                .display_order(2),
        )
        .arg(
            Arg::new_argument(PRINT_COMPILED_PROG)
                .help("Print the compiled program")
                .visible_alias("pcp"),
        )
        .arg(
            Arg::new_argument(PRINT_FINAL_PROG)
                .help("Print the final program after all pre-processing steps.")
                .visible_alias("pfp"),
        )
        .arg(
            Arg::new_argument(PRINT_OPTIMIZED_PROG)
                .help("Print the optimized program")
                .visible_alias("pop"),
        )
        .arg(
            Arg::new_argument(PRINT_INSTRUMENTED_PROG)
                .help("Print the instrumented program")
                .visible_alias("pip"),
        )
        .arg(
            Arg::new_argument(PRINT_MAIN_PROG)
                .help("Print the main program")
                .visible_alias("pmp"),
        )
        .arg(
            Arg::new_argument(PRINT_NORMALIZED_PROG)
                .help("Print the normalized program")
                .visible_alias("pnp"),
        )
        .arg(
            Arg::new_argument(PRINT_SPARSE_PROG)
                .help("Print the sparse program")
                .visible_alias("psp"),
        )
        .arg(
            Arg::new_argument(RUSTC_OPTIONS)
                .help("Options passed specifically to Rustc")
                .allow_invalid_utf8(true)
                .takes_value(true)
                .display_order(2),
        )
        .arg(
            Arg::new_argument(SOLANG_OPTIONS)
                .help("Options passed specifically to Solang")
                .allow_invalid_utf8(true)
                .takes_value(true)
                .display_order(2),
        )
        .arg(
            Arg::new_argument(SOLC_OPTIONS)
                .help("Options passed specifically to Solc")
                .allow_invalid_utf8(true)
                .takes_value(true)
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

    let include_dirs = match argms.values_of_os(INCLUDE_DIR) {
        None => vec![],
        Some(ss) => ss.into_iter().filter_map(|v| v.to_str()).collect(),
    };

    let include_files = match argms.values_of_os(INCLUDE_FILE) {
        None => vec![],
        Some(ss) => ss.into_iter().filter_map(|v| v.to_str()).collect(),
    };

    let rustc_user_options = match argms.values_of_os(RUSTC_OPTIONS) {
        None => vec![],
        Some(ss) => ss.into_iter().filter_map(|v| v.to_str()).collect(),
    };

    let solang_user_options = match argms.values_of_os(SOLANG_OPTIONS) {
        None => vec![],
        Some(ss) => ss.into_iter().filter_map(|v| v.to_str()).collect(),
    };

    let solc_user_options = match argms.values_of_os(SOLC_OPTIONS) {
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
        disable_instrumentation: argms.is_present(DISABLE_INSTRUMENT),
        disable_normalization: argms.is_present(DISABLE_NORMALIZE),
        disable_optimization: argms.is_present(DISABLE_OPTIMIZE),
        disable_printing: argms.is_present(DISABLE_PRINTING),
        generate_yul_statistics: argms.is_present(GENERATE_YUL_STATISTICS),
        include_dirs,
        include_files,
        instrument_code: argms.is_present(INSTRUMENT_BUG_ANNOTS),
        print_compiled_prog: argms.is_present(PRINT_COMPILED_PROG),
        print_final_prog: argms.is_present(PRINT_FINAL_PROG),
        print_instrumented_prog: argms.is_present(PRINT_INSTRUMENTED_PROG),
        print_optimized_prog: argms.is_present(PRINT_OPTIMIZED_PROG),
        print_main_prog: argms.is_present(PRINT_MAIN_PROG),
        print_normalized_prog: argms.is_present(PRINT_NORMALIZED_PROG),
        print_sparse_prog: argms.is_present(PRINT_SPARSE_PROG),
        rustc_options: rustc_user_options,
        solang_options: solang_user_options,
        solc_options: solc_user_options,
    }
}
