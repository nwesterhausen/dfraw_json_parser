use serde::{Deserialize, Serialize};

use crate::parser::{
    raws::{dimensions::Dimensions, RawModuleLocation},
    TypedJsonSerializable,
};

use super::DFTilePage;

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

    #[serde(rename = "filePath")]
    file_path: String,
    #[serde(rename = "tileDim")]
    tile_dim: Dimensions,
    #[serde(rename = "pageDim")]
    page_dim: Dimensions,
}

impl TypedJson {
    pub fn from(tile_page: &DFTilePage) -> Self {
        Self {
            identifier: tile_page.get_raw_header().get_identifier(),
            parent_raw: tile_page.get_raw_header().get_parent_raw(),
            object_id: tile_page.get_raw_header().get_object_id(),
            raw_module: tile_page.get_raw_header().get_raw_module(),
            raw_module_version: tile_page.get_raw_header().get_raw_module_version(),
            raw_module_found_in: tile_page.get_raw_header().get_dfraw_found_in(),
            raw_module_display: tile_page.get_raw_header().get_dfraw_display(),
            relative_path: tile_page.get_raw_header().get_dfraw_relative_path(),
            raw_type: tile_page.get_raw_header().get_raw_type(),
            name: tile_page.get_raw_header().get_identifier(),
            overwrites_raw: tile_page.get_raw_header().overwrites_raw.to_string(),
            tags: Vec::new(),

            tile_dim: tile_page.get_tile_dim(),
            page_dim: tile_page.get_page_dim(),
            file_path: tile_page.get_file_path(),
        }
    }
}

impl TypedJsonSerializable for DFTilePage {
    fn to_typed_json_string(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string(&TypedJson::from(self))
    }
}

impl TypedJsonSerializable for &DFTilePage {
    fn to_typed_json_string(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string(&TypedJson::from(self))
    }
}
