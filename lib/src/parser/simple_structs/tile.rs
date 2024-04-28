use serde::{Deserialize, Serialize};

use super::color::Color;

#[derive(ts_rs::TS)]
#[ts(export)]
#[allow(clippy::module_name_repetitions)]
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
/// Representation of a character tile (literally a single character) that is used in DF Classic
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

    pub fn get_character(&self) -> &str {
        &self.character
    }
    pub fn get_alt_character(&self) -> &str {
        &self.alt_character
    }
    pub fn get_color(&self) -> &Color {
        &self.color
    }
    pub fn get_glow_color(&self) -> &Color {
        &self.glow_color
    }

    pub fn with_character(mut self, character: &str) -> Self {
        self.set_character(character);
        self
    }
    pub fn with_alt_character(mut self, character: &str) -> Self {
        self.set_alt_character(character);
        self
    }
    pub fn with_color(mut self, color: &str) -> Self {
        self.set_color(color);
        self
    }
    pub fn with_glow_color(mut self, color: &str) -> Self {
        self.set_glow_color(color);
        self
    }
    pub fn with_glow_character(mut self, character: &str) -> Self {
        self.set_glow_character(character);
        self
    }
}
