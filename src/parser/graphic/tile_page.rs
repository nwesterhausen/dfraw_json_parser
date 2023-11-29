use std::path::PathBuf;

use serde::{Deserialize, Serialize};
use tracing::warn;

use crate::parser::{
<<<<<<< HEAD
<<<<<<<< HEAD:lib/src/parser/graphics/tile_page.rs
<<<<<<< HEAD:lib/src/parser/graphics/tile_page.rs
    clean_search_vec, helpers::build_object_id_from_pieces, object_types::ObjectType,
    serializer_helper, RawMetadata, RawObject, Searchable,
=======
========
    graphic::{dimensions::Dimensions, phf_table::TILE_PAGE_TAGS, tokens::TilePageTag},
>>>>>>>> 6f58260 (docs: add doc comments):src/parser/graphic/tile_page.rs
=======
    graphic::{dimensions::Dimensions, phf_table::TILE_PAGE_TAGS, tokens::TilePageTag},
>>>>>>> c2812957821240fff30b78553e73f23e904207e2
    helpers::build_object_id_from_pieces,
    helpers::serializer_helper,
    metadata::Metadata,
    object_type::ObjectType,
    raws::RawObject,
    searchable::{clean_search_vec, Searchable},
<<<<<<< HEAD
>>>>>>> 2b37a6f (refactor: expose 1 level down):src/parser/graphics/tile_page.rs
=======
>>>>>>> c2812957821240fff30b78553e73f23e904207e2
};

#[derive(ts_rs::TS)]
#[ts(export)]
#[allow(clippy::module_name_repetitions)]
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct TilePage {
    #[serde(skip_serializing_if = "serializer_helper::is_metadata_hidden")]
    metadata: Metadata,
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
    pub fn new(identifier: &str, metadata: &Metadata) -> Self {
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
    fn get_metadata(&self) -> &Metadata {
        &self.metadata
    }
    fn get_identifier(&self) -> &str {
        &self.identifier
    }
    fn get_name(&self) -> &str {
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
                warn!(
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

impl Searchable for TilePage {
    fn get_search_vec(&self) -> Vec<String> {
        let mut vec = Vec::new();

        vec.push(self.get_identifier().to_string());
        vec.push(format!("{:?}", self.get_type()));
        vec.push("tilePage".to_string());

        clean_search_vec(vec.as_slice())
    }
}
