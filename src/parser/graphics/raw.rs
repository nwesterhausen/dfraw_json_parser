use serde::{Deserialize, Serialize};

use crate::parser::{
    helpers::object_id::build_object_id_from_pieces,
    object_types::ObjectType,
    raws::{RawMetadata, RawObject},
    serializer_helper,
};

use super::{
    custom_extension::CustomGraphicExtension,
    phf_table::{CUSTOM_GRAPHIC_TAGS, GROWTH_TAGS, PLANT_GRAPHIC_TEMPLATES},
    sprite_graphic::SpriteGraphic,
    sprite_layer::SpriteLayer,
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

    #[serde(skip_serializing_if = "String::is_empty")]
    caste_identifier: String,
    kind: GraphicType,

    #[serde(skip_serializing_if = "Vec::is_empty")]
    sprites: Vec<SpriteGraphic>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    layers: Vec<(String, Vec<SpriteLayer>)>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    growths: Vec<(String, Vec<SpriteGraphic>)>,

    #[serde(skip_serializing_if = "Vec::is_empty")]
    custom_extensions: Vec<CustomGraphicExtension>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    tags: Vec<String>,

    #[serde(skip)]
    layer_mode: bool,
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
            if self.layers.is_empty() {
                self.layers.push((String::from("default"), Vec::new()));
            }
            self.layers.last_mut().unwrap().1.push(layer);
        }
    }
    fn parse_layer_condition_token(&mut self, key: &str, value: &str) {
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
            self.layer_mode = true;
            return;
        }

        // Check if key is LAYER meaning a new layer should be added to the current layer group
        if let "LAYER" = key {
            // Parse the value into a SpriteLayer
            self.parse_layer_from_value(value);
            self.layer_mode = true;
            return;
        }

        // Layers can be defined in groups.. for now we just ignore it
        if let "LAYER_GROUP" = key {
            self.layer_mode = true;
            return;
        }
        if let "END_LAYER_GROUP" = key {
            self.layer_mode = false;
            return;
        }

        // Right now we don't handle TREE_TILE
        if let "TREE_TILE" = key {
            return;
        }

        // Check if the key indicates a new growth.
        if let "GROWTH" = key {
            self.growths.push((String::from(value), Vec::new()));
            return;
        }

        // Check if the value is empty, which means we have a tag
        if value.is_empty() {
            self.tags.push(String::from(key));
            return;
        }

        // If the key is a custom extension, parse it into a CustomGraphicExtension and add it to the current sprite
        if let Some(extension_type) = CUSTOM_GRAPHIC_TAGS.get(key) {
            if let Some(custom_extension) =
                CustomGraphicExtension::from_value(*extension_type, value)
            {
                self.custom_extensions.push(custom_extension);
            } else {
                log::warn!(
                    "Graphic::parse_sprite_from_tag:_extension_type [{}] Failed to parse {},{} as CustomGraphicExtension",
                    self.identifier,
                    key,
                    value
                );
            }
            return;
        }

        // If the key is a growth token, parse it into a SpriteGraphic and add it to the current growth
        if let Some(_growth_type) = GROWTH_TAGS.get(key) {
            if let Some(sprite_graphic) = SpriteGraphic::from_token(key, value, graphic_type) {
                if let Some(growth) = self.growths.last_mut() {
                    growth.1.push(sprite_graphic);
                };
            } else {
                log::warn!(
                    "Graphic::parse_sprite_from_tag:_growth_type [{}] Failed to parse {},{} as SpriteGraphic",
                    self.identifier,
                    key,
                    value
                );
            }
            return;
        }
        // Check if the key is plant graphic template, which for now we accept only on growths
        if let Some(_plant_graphic_template) = PLANT_GRAPHIC_TEMPLATES.get(key) {
            if let Some(sprite_graphic) =
                SpriteGraphic::from_token(key, value, GraphicType::Template)
            {
                if let Some(growth) = self.growths.last_mut() {
                    growth.1.push(sprite_graphic);
                };
            } else {
                log::warn!(
                    "Graphic::parse_sprite_from_tag:_plant_graphic_template [{}] Failed to parse {},{} as SpriteGraphic",
                    self.identifier,
                    key,
                    value
                );
            }
            return;
        }

        // Check if we are in layer mode, and if so, parse the token as a layer condition
        if self.layer_mode {
            self.parse_layer_condition_token(key, value);
            return;
        }

        // Otherwise we can parse it for a sprite and report an error if that fails.
        if let Some(sprite_graphic) = SpriteGraphic::from_token(key, value, graphic_type) {
            self.sprites.push(sprite_graphic);
        } else {
            log::warn!(
                "Graphic::parse_sprite_from_tag:_from_token [{}] Failed to parse [{}:{}] as SpriteGraphic::{:?}",
                self.identifier,
                key,
                value,
                graphic_type
            );
        }
    }

    pub fn get_graphic_type(&self) -> GraphicType {
        self.kind
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
