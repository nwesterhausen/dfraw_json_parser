use std::any::Any;

use crate::parser::{metadata::Metadata, object_type::ObjectType, searchable::Searchable};

/// The `RawObject` trait is implemented by all raw objects. This trait is used
/// to provide a common interface for all raw objects, so that they can be
/// stored in a single vector. It also provides a common interface for parsing.
#[typetag::serde(tag = "type")]
pub trait RawObject: RawObjectToAny + Send + Sync + Searchable {
    /// Get the metadata for the raw.
    fn get_metadata(&self) -> &Metadata;
    /// Get the identifier of the raw.
    fn get_identifier(&self) -> &str;
    /// Returns true if the raw is empty.
    fn is_empty(&self) -> bool;
    /// Get the type of the raw.
    fn get_type(&self) -> &ObjectType;
    /// Parse a new tag from the raw file into this raw object.
    ///
    /// Arguments:
    ///
    /// * `key`: The key of the tag. The first part of a tag, before the colon.
    /// * `value`: The value of the tag. The second part of a tag, after the colon.
    /// The `value` might be empty, if there is no value after the colon.
    fn parse_tag(&mut self, key: &str, value: &str);
    /// Get the object ID of the raw.
    fn get_object_id(&self) -> &str;
    /// Get the name of the raw (if it has one).
    /// If no name is found, the identifier is returned instead.
    /// This is used for searching.
    fn get_name(&self) -> &str;
}

/// The `RawObjectToAny` trait is implemented by all raw objects. This trait is
/// used to be able to downcast a raw object to `Any`, so it can be downcast to
/// a specific raw object type.
pub trait RawObjectToAny: 'static {
    fn as_any(&self) -> &dyn Any;
}

/// The `RawObjectToAnyImpl` trait is implemented by all raw objects. This trait
/// is used to be able to downcast a raw object to `Any`, so it can be downcast
/// to a specific raw object type.
///
/// Make sure that the raw object reports to you the correct `ObjectType` that is
/// expected for the downcast.
impl<T: 'static> RawObjectToAny for T {
    fn as_any(&self) -> &dyn Any {
        self
    }
}
