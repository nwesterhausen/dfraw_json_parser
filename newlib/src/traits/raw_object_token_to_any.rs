#[allow(clippy::module_name_repetitions)]
#[typetag::serde(tag = "type")]
/// The `RawObjectToken` trait is implemented by all raw object tokens. This trait is used
/// to provide a common interface for all raw object tokens, so that they can be
/// stored in a single vector. It also provides a common interface for parsing.
pub trait RawObjectToken: RawObjectTokenToAny + std::fmt::Debug + Send + Sync {}

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
