use serde::{Deserialize, Serialize};

use super::color::Color;

#[derive(ts_rs::TS)]
#[ts(export)]
#[allow(clippy::module_name_repetitions)]
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
/// This struct represents how to draw the object on screen without graphics.
pub struct Tile {
    /// The character to use to draw the object. (e.g. `☼`)
    character: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    /// An alternate character to draw the object. It will flash between the two.
    alt_character: String,
    #[serde(skip_serializing_if = "Color::is_default")]
    /// The color to draw the object.
    color: Color,
    #[serde(skip_serializing_if = "String::is_empty")]
    /// The character to use to draw the object when it is glowing. (e.g. `☼`)
    glow_character: String,
    #[serde(skip_serializing_if = "Color::is_default")]
    /// The color to draw the object when it is glowing.
    glow_color: Color,
}

impl Tile {
    /// Set the `character` field.
    ///
    /// Arguments:
    ///
    /// * `character`: A string slice that holds the value of the `character` field.
    pub fn set_character(&mut self, character: &str) {
        self.character = String::from(character);
    }
    /// Set the `alt_character` field.
    ///
    /// Arguments:
    ///
    /// * `character`: A string slice that holds the value of the `alt_character` field.
    pub fn set_alt_character(&mut self, character: &str) {
        self.alt_character = String::from(character);
    }
    /// Set the `color` field. This in turn calls `Color::from_value` to set the `color` field.
    ///
    /// Arguments:
    ///
    /// * `color`: A string slice that holds the value of the `color` field.
    pub fn set_color(&mut self, color: &str) {
        self.color = Color::from_value(color);
    }
    /// Set the `color` field using an exact `Color` struct.
    ///
    /// Arguments:
    ///
    /// * `color`: A `Color` struct to set the `color` field to.
    pub fn set_color_exact(&mut self, color: &Color) {
        self.color = color.clone();
    }
    /// Set the `glow_color` field. This in turn calls `Color::from_value` to set the `glow_color` field.
    ///
    /// Arguments:
    ///
    /// * `color`: A string slice that holds the value of the `glow_color` field.
    pub fn set_glow_color(&mut self, color: &str) {
        self.glow_color = Color::from_value(color);
    }
    /// Set the `glow_color` field using an exact `Color` struct.
    ///
    /// Arguments:
    ///
    /// * `color`: A `Color` struct to set the `color` field to.
    pub fn set_glow_color_exact(&mut self, color: &Color) {
        self.color = color.clone();
    }
    /// Set the `glow_character` field.
    ///
    /// Arguments:
    ///
    /// * `character`: A string slice that holds the value of the `glow_character` field.
    pub fn set_glow_character(&mut self, character: &str) {
        self.glow_character = String::from(character);
    }
    /// Check if the `Tile` struct is empty/default.
    ///
    /// Returns:
    ///
    /// * `bool`: True if the `Tile` struct is empty, false otherwise.
    pub fn is_default(&self) -> bool {
        self.character.is_empty()
            && self.alt_character.is_empty()
            && self.color.is_default()
            && self.glow_character.is_empty()
            && self.glow_color.is_default()
    }
}
