use serde::{Deserialize, Serialize};

use crate::parser::{raws::RawModuleLocation, TypedJsonSerializable};

use super::{DFGraphic, Kind, SpriteGraphic};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TypedJson {
    // Common Raw file Things
    identifier: String,
    #[serde(rename = "parentRaw")]
    parent_raw: String,
    #[serde(rename = "rawModule")]
    raw_module: String,
    #[serde(rename = "moduleVersion")]
    raw_module_version: String,
    #[serde(rename = "moduleSourceDirectory")]
    raw_module_found_in: RawModuleLocation,
    #[serde(rename = "moduleDisplayName")]
    raw_module_display: String,
    #[serde(rename = "rawType")]
    raw_type: String,
    #[serde(rename = "rawRelativePath")]
    relative_path: String,
    #[serde(rename = "objectId")]
    object_id: String,
    #[serde(rename = "overwriteRaw")]
    overwrites_raw: String,

    tags: Vec<usize>,
    name: String,

    target_identifier: String,
    caste_identifier: String,
    pub kind: Kind,
    graphics: Vec<SpriteGraphic>,
}

impl TypedJson {
    pub fn from(graphic: &DFGraphic) -> Self {
        Self {
            identifier: format!("graphic_{}", graphic.get_raw_header().get_identifier()),
            parent_raw: graphic.get_raw_header().get_parent_raw(),
            object_id: graphic.get_raw_header().get_object_id(),
            raw_module: graphic.get_raw_header().get_raw_module(),
            raw_module_version: graphic.get_raw_header().get_raw_module_version(),
            raw_module_found_in: graphic.get_raw_header().get_dfraw_found_in(),
            raw_module_display: graphic.get_raw_header().get_dfraw_display(),
            relative_path: graphic.get_raw_header().get_dfraw_relative_path(),
            raw_type: graphic.get_raw_header().get_raw_type(),
            name: graphic.get_raw_header().get_identifier(),
            overwrites_raw: graphic.get_raw_header().overwrites_raw.to_string(),
            tags: Vec::new(),

            target_identifier: graphic.get_raw_header().get_identifier(),
            caste_identifier: graphic.get_caste_identifier(),
            kind: graphic.get_kind(),
            graphics: graphic.get_graphics(),
        }
    }
}

impl TypedJsonSerializable for DFGraphic {
    fn to_typed_json_string(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string(&TypedJson::from(self))
    }
}

impl TypedJsonSerializable for &DFGraphic {
    fn to_typed_json_string(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string(&TypedJson::from(self))
    }
}
