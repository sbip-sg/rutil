//! Module containing declarations of useful traits.

/// Trait declaring utilities for printing.
pub trait Printing<'a, S> {
    /// Print to string
    fn print_to_string(&self) -> String;
}
