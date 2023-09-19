use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use super::{raw_object_kind::RawObjectKind, tags::DFTag};

// The metadata for a DF Raw. This includes information about the raw module
// the raw is from, and which file contains the raw.
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct RawMetadata {
    // The name of the raw module the raw is from.
    module_name: String,
    // The version of the raw module the raw is from.
    module_version: String,
    // The path to the file containing the raw.
    raw_file_path: String,
    // The raw identifier (as described at the top of the raw text file).
    raw_identifier: String,
    // The type of raw (creature, plant, etc).
    // Example: [OBJECT:TYPE]
    raw_type: RawObjectKind,
}

// A very generic raw object.
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DFRaw {
    // The raw identifier (first line of the raw includes type and identifier)
    // Example: [TYPE:IDENTIFIER]
    identifier: String,
    // The type of raw (creature, plant, etc).
    raw_type: RawObjectKind,
    // The metadata for the raw.
    metadata: RawMetadata,
    // The raw properties.
    properties: RawProperties,
}

// The properties of a raw object.
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct RawProperties {
    // Any free-standing tags in the raw.
    // Example: [TAG]
    tags: Vec<DFTag>,
    // Any key-value pairs in the raw.
    // Example: [KEY:VALUE]
    basic_properties: HashMap<String, String>,
    // Any other values in the raw.
    // I expect as we parse the raws, we'll need to expand our properties.
    other: Vec<String>,
}

impl DFRaw {
    pub fn empty() -> Self {
        Self {
            identifier: String::new(),
            raw_type: RawObjectKind::None,
            metadata: RawMetadata {
                module_name: String::new(),
                module_version: String::new(),
                raw_file_path: String::new(),
                raw_identifier: String::new(),
                raw_type: RawObjectKind::None,
            },
            properties: RawProperties {
                tags: Vec::new(),
                basic_properties: HashMap::new(),
                other: Vec::new(),
            },
        }
    }

    pub(crate) fn get_identifier(&self) -> &str {
        &self.identifier
    }
    pub(crate) fn is_empty(&self) -> bool {
        self.identifier.is_empty()
    }
}
