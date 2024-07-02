//! This module contains the `TagOperations` trait, which is used to parse tokens from raw files.

use crate::metadata::TagComplexity;

/// The `TokenOperations` trait is used to parse tokens from raw files.
pub trait TagOperations: Sized + Sync + Send {
    /// Get the complexity of the token
    fn get_complexity(&self) -> TagComplexity;
    /// Parse an unknown token with a key and a (possibly empty) value(s).
    fn parse(key: &str, value: &str) -> Option<Self>;
    /// Parse a token with a key and a single value.
    fn parse_simple_token(&self, value: &str) -> Option<Self>;
    /// Parse a token with a key and multiple values.
    fn parse_complex_token(&self, values: &[&str]) -> Option<Self>;
}
