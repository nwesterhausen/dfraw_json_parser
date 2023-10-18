use std::path::PathBuf;

use serde::{Deserialize, Serialize};

use crate::parser::{
    helpers::object_id::build_object_id_from_pieces,
    object_types::ObjectType,
    raws::{RawMetadata, RawObject},
    serializer_helper,
};

use super::{dimensions::Dimensions, phf_table::TILE_PAGE_TAGS, tokens::TilePageTag};

#[derive(ts_rs::TS)]
#[ts(export)]
#[allow(clippy::module_name_repetitions)]
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct TilePage {
    #[serde(skip_serializing_if = "serializer_helper::is_metadata_hidden")]
    metadata: RawMetadata,
    identifier: String,
    object_id: String,

    file: PathBuf,
    tile_dim: Dimensions,
    page_dim: Dimensions,
}

impl TilePage {
    pub fn empty() -> Self {
        Self::default()
    }
    pub fn new(identifier: &str, metadata: &RawMetadata) -> Self {
        Self {
            identifier: String::from(identifier),
            metadata: metadata.clone(),
            object_id: build_object_id_from_pieces(metadata, identifier, &ObjectType::TilePage),
            ..Self::default()
        }
    }
}

#[typetag::serde]
impl RawObject for TilePage {
    fn get_metadata(&self) -> &RawMetadata {
        &self.metadata
    }
    fn get_identifier(&self) -> &str {
        &self.identifier
    }
    fn is_empty(&self) -> bool {
        self.identifier.is_empty()
    }
    fn get_type(&self) -> &ObjectType {
        &ObjectType::TilePage
    }

    fn parse_tag(&mut self, key: &str, value: &str) {
        match TILE_PAGE_TAGS.get(key).unwrap_or(&TilePageTag::Unknown) {
            TilePageTag::File => {
                let relative_path: PathBuf = value.split('/').collect();
                let raw_path = PathBuf::from(self.metadata.get_raw_file_path());
                self.file = raw_path.parent().unwrap_or(&raw_path).join(relative_path);
            }
            TilePageTag::TileDim => {
                self.tile_dim = Dimensions::from_token(value);
            }
            TilePageTag::PageDim => {
                self.page_dim = Dimensions::from_token(value);
            }
            TilePageTag::Unknown => {
                log::warn!(
                    "Failed to parse {} as TilePageTag for {}",
                    key,
                    self.get_object_id()
                );
            }
        }
    }

    fn get_object_id(&self) -> &str {
        &self.object_id
    }
}
