use crate::metadata::TokenComplexity;

#[allow(clippy::module_name_repetitions)]
#[typetag::serde(tag = "type")]
/// The `RawObjectToken` trait is implemented by all raw object tokens. This trait is used
/// to provide a common interface for all raw object tokens, so that they can be
/// stored in a single vector. It also provides a common interface for parsing.
pub trait RawObjectToken: RawObjectTokenToAny + std::fmt::Debug + Send + Sync {
    /// Get the complexity of the token.
    ///
    /// The complexity helps determine how the token is parsed.
    fn get_complexity(&self) -> TokenComplexity;
    /// Parse a token from a key and value (if any).
    ///
    /// Should create a new token of Self and return it.
    ///
    /// Arguments:
    ///
    /// * `key`: The key of the tag. The first part of a tag, before the colon.
    /// * `value`: The value of the tag. The second part of a tag, after the colon.
    /// The `value` might be empty, if there is no value after the colon.
    ///
    /// Returns:
    ///
    /// * `Option<Self>`: The token that was parsed, or None if the token could not be parsed.
    fn parse_token(key: &str, value: &str) -> Option<Self>
    where
        Self: Sized;
}

impl std::fmt::Display for dyn RawObjectToken {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

/// The `RawObjectTokenToAny` trait is implemented by all raw object tokens. This trait is
/// used to be able to downcast a raw object token to `Any`, so it can be downcast to
/// a specific raw object token type.
pub trait RawObjectTokenToAny: 'static {
    fn as_any(&self) -> &dyn std::any::Any;
}

impl<T: 'static> RawObjectTokenToAny for T {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}
