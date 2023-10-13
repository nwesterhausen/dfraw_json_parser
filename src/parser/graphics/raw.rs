use serde::{Deserialize, Serialize};

use crate::parser::{
    object_types::ObjectType,
    raws::{build_object_id_from_pieces, RawMetadata, RawObject},
    serializer_helper,
};

use super::{
    phf_table::LAYER_CONDITION_TAGS, sprite_graphic::SpriteGraphic, sprite_layer::SpriteLayer,
    tokens::GraphicType,
};

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
    layers: Vec<(String, Vec<SpriteLayer>)>,
}

impl Graphic {
    pub fn empty() -> Self {
        Self::default()
    }
    pub fn new(identifier: &str, metadata: &RawMetadata, graphic_type: GraphicType) -> Self {
        Self {
            identifier: String::from(identifier),
            metadata: metadata.clone(),
            object_id: build_object_id_from_pieces(metadata, identifier, &ObjectType::Graphics),
            kind: graphic_type,
            ..Self::default()
        }
    }
    fn parse_layer_set_from_value(&mut self, value: &str) {
        self.layers.push((String::from(value), Vec::new()));
    }
    fn parse_layer_from_value(&mut self, value: &str) {
        if let Some(layer) = SpriteLayer::parse_layer_from_value(value) {
            self.layers.last_mut().unwrap().1.push(layer);
        }
    }
    fn parse_condition_token(&mut self, key: &str, value: &str) {
        // Conditions get attached to the last layer in the last layer group
        if let Some(layer) = self.layers.last_mut().unwrap().1.last_mut() {
            layer.parse_condition_token(key, value);
        } else {
            log::warn!(
                "Graphic::parse_condition_token: [{}] Failed to parse {}:{} as LayerCondition",
                self.identifier,
                key,
                value
            );
        }
    }
    pub fn parse_sprite_from_tag(&mut self, key: &str, value: &str, graphic_type: GraphicType) {
        // Check if key is LAYER_SET meaning a new layer group is starting
        if let "LAYER_SET" = key {
            // Parse the value into a SpriteLayer
            self.parse_layer_set_from_value(value);
            return;
        }

        // Check if key is LAYER meaning a new layer should be added to the current layer group
        if let "LAYER" = key {
            // Parse the value into a SpriteLayer
            self.parse_layer_from_value(value);
            return;
        }

        // Check if the key is in the LAYER_CONDITION_TAGS table, meaning it is a condition
        if let Some(_condition) = LAYER_CONDITION_TAGS.get(key) {
            // Parse the value into a SpriteLayer
            self.parse_condition_token(key, value);
            return;
        }

        // Otherwise we can parse it for a sprite and report an error if that fails.
        if let Some(sprite_graphic) = SpriteGraphic::from_token(key, value, graphic_type) {
            self.graphics.push(sprite_graphic);
        } else {
            log::warn!(
                "Graphic::parse_sprite_from_tag: [{}] Failed to parse {},{} as SpriteGraphic",
                self.identifier,
                key,
                value
            );
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
        // Any tags should just be able to be handled by the sprite graphic, but it needs to call the right function
        log::warn!(
            "Graphics tag attempted parse with wrong method: {}:{} for {}",
            key,
            value,
            self.get_identifier()
        );
    }

    fn get_object_id(&self) -> &str {
        &self.object_id
    }
}