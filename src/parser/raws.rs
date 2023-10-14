use std::{any::Any, path::Path};

use serde::{Deserialize, Serialize};
use slug::slugify;

use super::{
    module_info_file::ModuleInfoFile, object_types::ObjectType, raw_locations::RawModuleLocation,
};

#[typetag::serde(tag = "type")]
pub trait RawObject: RawObjectToAny {
    fn get_metadata(&self) -> &RawMetadata;
    fn get_identifier(&self) -> &str;
    fn is_empty(&self) -> bool;
    fn get_type(&self) -> &ObjectType;
    fn parse_tag(&mut self, key: &str, value: &str);
    fn get_object_id(&self) -> &str;
}

pub trait RawObjectToAny: 'static {
    fn as_any(&self) -> &dyn Any;
}

impl<T: 'static> RawObjectToAny for T {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

// The metadata for a DF Raw. This includes information about the raw module
// the raw is from, and which file contains the raw.
#[derive(Serialize, Deserialize, Clone, Debug, Default)]
#[serde(rename_all = "camelCase")]
#[derive(ts_rs::TS)]
#[ts(export)]
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
    object_type: ObjectType,
    // The location of the owning raw module
    // i.e. installed_mods, mods, or vanilla
    raw_module_location: RawModuleLocation,
    // Optionally hide or unhide from exporting
    // By default will be hidden
    #[serde(skip)]
    hidden: bool,
}

impl RawMetadata {
    pub fn new<P: AsRef<Path>>(
        module_info: &ModuleInfoFile,
        object_type: &ObjectType,
        raw_identifier: &str,
        raw_file_path: &P,
        attach_metadata_to_raws: bool,
    ) -> Self {
        Self {
            module_name: module_info.get_name(),
            module_version: module_info.get_version(),
            raw_file_path: String::from(raw_file_path.as_ref().to_str().unwrap_or_default()),
            raw_identifier: String::from(raw_identifier),
            object_type: object_type.clone(),
            raw_module_location: module_info.get_location(),
            hidden: !attach_metadata_to_raws,
        }
    }
    /// Used only for serialization
    pub fn is_hidden(&self) -> bool {
        self.hidden
    }
    pub fn get_raw_identifier(&self) -> &str {
        &self.raw_identifier
    }
    pub fn get_module_name(&self) -> &str {
        &self.module_name
    }
    pub fn get_module_numerical_version(&self) -> &str {
        &self.module_version
    }
    pub fn get_module_version(&self) -> &str {
        &self.module_version
    }
    pub fn get_raw_file_path(&self) -> &str {
        &self.raw_file_path
    }
}

pub fn build_object_id_from_pieces(
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
