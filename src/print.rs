//! Module to provide printing utilities.

use std::fmt::Display;

/// Print a vector to string by a default printing function.
///
/// Elements are separated by a `separator` string.
pub fn print_vector_to_string<T>(items: &[T], separator: &str) -> String
where
    T: Display,
{
    items
        .into_iter()
        .map(|elem| format!("{}", elem))
        .collect::<Vec<String>>()
        .join(separator)
}

/// Print a vector to string using a printer.
///
/// Elements are separated by a `separator` string.
pub fn print_vector_by_printer<T>(
    items: &[T],
    printer: &dyn Fn(&T) -> String,
    separator: &str,
) -> String
where
    T: Display,
{
    items
        .into_iter()
        .map(|elem| printer(elem))
        .collect::<Vec<String>>()
        .join(separator)
}
