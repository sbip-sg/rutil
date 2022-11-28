//! Module to implement additional functionalities of `Result` data structure.

use anyhow::Result;

/// Trait for converting a result.
pub trait IntoResult {
    /// Convert into an `Ok` result.
    fn into_ok(self) -> Result<Self>
    where
        Self: Sized,
    {
        Ok(self)
    }
}
