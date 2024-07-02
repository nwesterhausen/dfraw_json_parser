//! Used to determine if a token is within a raw object.

use super::RawObject;

#[typetag::serialize]
/// The `RawObjectToken` trait is implemented by all raw object tokens. This trait is used
/// to provide a common interface for all raw object tokens, so that they can be
/// stored in a single vector. It also provides a common interface for parsing.
pub trait RawObjectToken<T: RawObject> {
    /// Check if the token is within the object.
    fn is_within(&self, object: &T) -> bool;
}
