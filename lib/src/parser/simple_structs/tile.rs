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
    /// Sets the character of the tile
    ///
    /// # Arguments
    ///
    /// * `character` - The character to set
    pub fn set_character(&mut self, character: &str) {
        self.character = String::from(character);
    }
    /// Sets the alternate character of the tile
    ///
    /// # Arguments
    ///
    /// * `character` - The character to set
    pub fn set_alt_character(&mut self, character: &str) {
        self.alt_character = Some(String::from(character));
    }
    /// Sets the color of the tile
    ///
    /// # Arguments
    ///
    /// * `color` - The color to set
    pub fn set_color(&mut self, color: &str) {
        self.color = Some(Color::from_value(color));
    }
    /// Sets the glow color of the tile
    ///
    /// # Arguments
    ///
    /// * `color` - The color to set
    pub fn set_glow_color(&mut self, color: &str) {
        self.glow_color = Some(Color::from_value(color));
    }
    /// Sets the glow character of the tile
    ///
    /// # Arguments
    ///
    /// * `character` - The character to set
    pub fn set_glow_character(&mut self, character: &str) {
        self.glow_character = Some(String::from(character));
    }
    /// Returns whether the tile is the default tile
    ///
    /// # Returns
    ///
    /// * `bool` - Whether the tile is the default tile
    pub fn is_default(&self) -> bool {
        self.character.is_empty()
            && self.alt_character.is_none()
            && self.color.is_none()
            && self.glow_character.is_none()
            && self.glow_color.is_none()
    }
    /// Returns the character of the tile
    ///
    /// # Returns
    ///
    /// * `&str` - The character of the tile
    pub fn get_character(&self) -> &str {
        &self.character
    }
    /// Returns the alternate character of the tile
    ///
    /// # Returns
    ///
    /// * `&str` - The alternate character of the tile
    pub fn get_alt_character(&self) -> &str {
        self.alt_character
            .as_ref()
            .map_or("", |alt_character| alt_character)
    }
    /// Returns the color of the tile
    ///
    /// # Returns
    ///
    /// * `Color` - The color of the tile
    #[must_use]
    pub fn get_color(&self) -> Color {
        self.color.as_ref().map_or_else(
            || {
                tracing::info!("Had to coerce a default color for a tile");
                Color::default()
            },
            std::clone::Clone::clone,
        )
    }
    /// Returns the glow color of the tile
    ///
    /// # Returns
    ///
    /// * `Color` - The glow color of the tile
    #[must_use]
    pub fn get_glow_color(&self) -> Color {
        self.glow_color.as_ref().map_or_else(
            || {
                tracing::info!("Had to coerce a default color for a tile");
                Color::default()
            },
            std::clone::Clone::clone,
        )
    }
    /// Sets the character of the tile and returns the tile
    ///
    /// # Arguments
    ///
    /// * `character` - The character to set
    ///
    /// # Returns
    ///
    /// * `Self` - The modified tile
    ///
    /// # Example
    ///
    /// ```
    /// use dfraw_json_parser::parser::Tile;
    ///
    /// let tile = Tile::default().with_character("a");
    /// ```
    #[must_use]
    pub fn with_character(mut self, character: &str) -> Self {
        self.set_character(character);
        self
    }
    /// Sets the alternate character of the tile and returns the tile
    ///
    /// # Arguments
    ///
    /// * `character` - The character to set
    ///
    /// # Returns
    ///
    /// * `Self` - The modified tile
    #[must_use]
    pub fn with_alt_character(mut self, character: &str) -> Self {
        self.set_alt_character(character);
        self
    }
    /// Sets the color of the tile and returns the tile
    ///
    /// # Arguments
    ///
    /// * `color` - The color to set
    ///
    /// # Returns
    ///
    /// * `Self` - The modified tile
    #[must_use]
    pub fn with_color(mut self, color: &str) -> Self {
        self.set_color(color);
        self
    }
    /// Sets the glow color of the tile and returns the tile
    ///
    /// # Arguments
    ///
    /// * `color` - The color to set
    ///
    /// # Returns
    ///
    /// * `Self` - The modified tile
    #[must_use]
    pub fn with_glow_color(mut self, color: &str) -> Self {
        self.set_glow_color(color);
        self
    }
    /// Sets the glow character of the tile and returns the tile
    ///
    /// # Arguments
    ///
    /// * `character` - The character to set
    ///
    /// # Returns
    ///
    /// * `Self` - The modified tile
    #[must_use]
    pub fn with_glow_character(mut self, character: &str) -> Self {
        self.set_glow_character(character);
        self
    }
}
