use slug::slugify;

use crate::parser::{ObjectType, RawMetadata};

/// The function `build_object_id_from_pieces` takes in metadata, an identifier, and a raw type, and
/// returns a formatted string. This is a convenience function for building object IDs so they all
/// follow the same format.
///
/// Arguments:
///
/// * `metadata`: A reference to a `RawMetadata` object.
/// * `identifier`: A string representing the identifier of the object.
/// * `raw_type`: The `raw_type` parameter is of type `ObjectType`.
///
/// Returns:
///
/// a string.
#[must_use] pub fn build_object_id_from_pieces(
    metadata: &RawMetadata,
    identifier: &str,
    raw_type: &ObjectType,
) -> String {
    format!(
        "{raw_parent_id}-{raw_type}-{raw_id}-{module_name}{module_version}",
        raw_id = slugify(identifier),
        raw_parent_id = slugify(metadata.get_raw_identifier()),
        module_version = metadata.get_module_numerical_version(),
        module_name = slugify(metadata.get_module_name()),
    )
}
