use serde::{Deserialize, Serialize};

use crate::parser::{
    object_types::ObjectType,
    raws::{build_object_id_from_pieces, RawMetadata, RawObject},
    serializer_helper,
};

use super::{sprite_graphic::SpriteGraphic, tokens::GraphicType};

#[derive(ts_rs::TS)]
#[ts(export)]
#[allow(clippy::module_name_repetitions)]
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct Graphic {
    #[serde(skip_serializing_if = "serializer_helper::is_metadata_hidden")]
    metadata: RawMetadata,
    identifier: String,
    object_id: String,

    caste_identifier: String,
    kind: GraphicType,
    graphics: Vec<SpriteGraphic>,
}

impl Graphic {
    pub fn empty() -> Self {
        Self::default()
    }
    pub fn new(identifier: &str, metadata: &RawMetadata) -> Self {
        Self {
            identifier: String::from(identifier),
            metadata: metadata.clone(),
            object_id: build_object_id_from_pieces(metadata, identifier, &ObjectType::Graphics),
            ..Self::default()
        }
    }
}

#[typetag::serde]
impl RawObject for Graphic {
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
        &ObjectType::Graphics
    }

    fn parse_tag(&mut self, key: &str, value: &str) {
        // Any tags should just be able to be handled by the sprite graphic
        if let Some(sprite_graphic) = SpriteGraphic::from_token(key, value) {
            self.graphics.push(sprite_graphic);
        } else {
            log::warn!(
                "Failed to parse {} as SpriteGraphic for {}",
                value,
                self.identifier
            );
        }
    }

    fn get_object_id(&self) -> &str {
        &self.object_id
    }
}
