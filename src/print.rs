//! Module to provide printing utilities.

/// Trait to providing printing functions for Vector type.
pub trait PrintVec {
    /// Print a vector to string, elements are separated by a separator.
    fn print_to_string(&self, separator: &str) -> String;

    /// Print a vector to string, elements are separated by a comma.
    fn print_to_comma_separated_string(&self) -> String {
        self.print_to_string(", ")
    }

    /// Print a vector to string, elements are separated by a semicolon.
    fn print_to_semicolon_separated_string(&self) -> String {
        self.print_to_string("; ")
    }

    /// Print a vector to string, elements are separated by a new line.
    fn print_to_newline_separated_string(&self) -> String {
        self.print_to_string("\n")
    }
}
