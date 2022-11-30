//! Module to provide printing utilities.

use std::fmt::Display;

/// Module providing printing functions for Vector type.

/// Print a vector to string, elements are separated by a separator.
pub fn print_vec_to_string<T>(items: &[T], separator: &str) -> String
where
    T: Display,
{
    items
        .into_iter()
        .map(|elem| format!("{}", elem))
        .collect::<Vec<String>>()
        .join(separator)
}

/// Print a vector to string, elements are separated by a comma.
pub fn print_vec_to_comma_separated_string<T>(items: &[T]) -> String
where
    T: Display + Iterator,
{
    print_vec_to_string(items, ", ")
}

/// Print a vector to string, elements are separated by a semicolon.
pub fn print_vec_to_semicolon_separated_string<T>(items: &[T]) -> String
where
    T: Display + Iterator,
{
    print_vec_to_string(items, "; ")
}

/// Print a vector to string, elements are separated by a new line.
pub fn print_vec_to_newline_separated_string<T>(items: &[T]) -> String
where
    T: Display + Iterator,
{
    print_vec_to_string(items, "\n")
}
