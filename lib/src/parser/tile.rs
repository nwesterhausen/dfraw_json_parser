use serde::{Deserialize, Serialize};

use super::color::Color;

#[derive(ts_rs::TS)]
#[ts(export)]
#[allow(clippy::module_name_repetitions)]
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct Tile {
    character: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    alt_character: String,
    #[serde(skip_serializing_if = "Color::is_default")]
    color: Color,
    #[serde(skip_serializing_if = "String::is_empty")]
    glow_character: String,
    #[serde(skip_serializing_if = "Color::is_default")]
    glow_color: Color,
}

impl Tile {
    pub fn set_character(&mut self, character: &str) {
        self.character = String::from(character);
    }
    pub fn set_alt_character(&mut self, character: &str) {
        self.alt_character = String::from(character);
    }
    pub fn set_color(&mut self, color: &str) {
        self.color = Color::from_value(color);
    }
    pub fn set_glow_color(&mut self, color: &str) {
        self.glow_color = Color::from_value(color);
    }
    pub fn set_glow_character(&mut self, character: &str) {
        self.glow_character = String::from(character);
    }
    pub fn is_default(&self) -> bool {
        self.character.is_empty()
            && self.alt_character.is_empty()
            && self.color.is_default()
            && self.glow_character.is_empty()
            && self.glow_color.is_default()
    }
}
