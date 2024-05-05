use serde::{Deserialize, Serialize};

use super::color::Color;

#[allow(clippy::module_name_repetitions)]
#[derive(Serialize, Deserialize, Debug, Clone, Default, specta::Type)]
#[serde(rename_all = "camelCase")]
/// Representation of a character tile (literally a single character) that is used in DF Classic
pub struct Tile {
    character: String,
    alt_character: Option<String>,
    color: Option<Color>,
    glow_character: Option<String>,
    glow_color: Option<Color>,
}

impl Tile {
    pub fn set_character(&mut self, character: &str) {
        self.character = String::from(character);
    }
    pub fn set_alt_character(&mut self, character: &str) {
        self.alt_character = Some(String::from(character));
    }
    pub fn set_color(&mut self, color: &str) {
        self.color = Some(Color::from_value(color));
    }
    pub fn set_glow_color(&mut self, color: &str) {
        self.glow_color = Some(Color::from_value(color));
    }
    pub fn set_glow_character(&mut self, character: &str) {
        self.glow_character = Some(String::from(character));
    }
    pub fn is_default(&self) -> bool {
        self.character.is_empty()
            && self.alt_character.is_none()
            && self.color.is_none()
            && self.glow_character.is_none()
            && self.glow_color.is_none()
    }

    pub fn get_character(&self) -> &str {
        &self.character
    }
    pub fn get_alt_character(&self) -> &str {
        if let Some(alt_character) = &self.alt_character {
            alt_character
        } else {
            ""
        }
    }
    #[must_use]
    pub fn get_color(&self) -> Color {
        if let Some(color) = &self.color {
            color.clone()
        } else {
            tracing::info!("Had to coerce a default color for a tile");
            Color::default()
        }
    }
    #[must_use]
    pub fn get_glow_color(&self) -> Color {
        if let Some(color) = &self.glow_color {
            color.clone()
        } else {
            tracing::info!("Had to coerce a default color for a tile");
            Color::default()
        }
    }

    #[must_use]
    pub fn with_character(mut self, character: &str) -> Self {
        self.set_character(character);
        self
    }
    #[must_use]
    pub fn with_alt_character(mut self, character: &str) -> Self {
        self.set_alt_character(character);
        self
    }
    #[must_use]
    pub fn with_color(mut self, color: &str) -> Self {
        self.set_color(color);
        self
    }
    #[must_use]
    pub fn with_glow_color(mut self, color: &str) -> Self {
        self.set_glow_color(color);
        self
    }
    #[must_use]
    pub fn with_glow_character(mut self, character: &str) -> Self {
        self.set_glow_character(character);
        self
    }
}
