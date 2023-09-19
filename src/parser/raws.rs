use std::{collections::HashMap, path::Path};

use serde::{Deserialize, Serialize};

use super::{
    mod_info_file::ModuleInfoFile, raw_locations::RawModuleLocation,
    raw_object_kind::RawObjectKind, tags::DFTag,
};

// The metadata for a DF Raw. This includes information about the raw module
// the raw is from, and which file contains the raw.
#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RawMetadata {
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
    // The location of the owning raw module
    // i.e. installed_mods, mods, or vanilla
    raw_module_location: RawModuleLocation,
}

impl RawMetadata {
    pub fn new<P: AsRef<Path>>(
        module_info: &ModuleInfoFile,
        raw_variant: &RawObjectKind,
        raw_identifier: &str,
        raw_file_path: &P,
    ) -> Self {
        Self {
            module_name: module_info.get_name(),
            module_version: module_info.get_version(),
            raw_file_path: String::from(raw_file_path.as_ref().to_str().unwrap_or_default()),
            raw_identifier: String::from(raw_identifier),
            raw_type: raw_variant.clone(),
            raw_module_location: module_info.get_location(),
        }
    }
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
pub struct RawProperties {
    // Any free-standing tags in the raw.
    // Example: [TAG]
    tags: Vec<DFTag>,
    // Any key-value pairs in the raw.
    // Example: [KEY:VALUE]
    basic_properties: HashMap<String, String>,
    // Sub-properties for castes
    castes: HashMap<String, RawProperties>,
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
                raw_module_location: RawModuleLocation::Unknown,
            },
            properties: RawProperties {
                tags: Vec::new(),
                basic_properties: HashMap::new(),
                other: Vec::new(),
                castes: HashMap::new(),
            },
        }
    }
    pub fn new(identifier: &str, variant: RawObjectKind, metadata: &RawMetadata) -> Self {
        Self {
            identifier: String::from(identifier),
            raw_type: variant,
            metadata: metadata.clone(),
            properties: RawProperties {
                tags: Vec::new(),
                basic_properties: HashMap::new(),
                other: Vec::new(),
                castes: HashMap::new(),
            },
        }
    }

    pub fn get_identifier(&self) -> &str {
        &self.identifier
    }
    pub fn is_empty(&self) -> bool {
        self.identifier.is_empty()
    }
    pub fn set_tags(&mut self, tags: Vec<DFTag>) {
        self.properties.tags = tags;
    }
    pub(crate) fn set_castes(&mut self, castes_temp: HashMap<String, RawProperties>) {
        self.properties.castes = castes_temp;
    }
    pub(crate) fn add_basic_property(&mut self, key: String, value: String) {
        self.properties.basic_properties.insert(key, value);
    }
    pub(crate) fn add_other_property(&mut self, property: String) {
        self.properties.other.push(property);
    }
}
