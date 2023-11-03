use serde::{Deserialize, Serialize};

#[derive(ts_rs::TS)]
#[ts(export)]
#[allow(clippy::module_name_repetitions)]
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct Color {
    /// The foreground color value
    foreground: u8,
    /// The background color value
    background: u8,
    /// The brightness value
    brightness: u8,
}

impl Color {
    /// The function `from_value` takes a string value and splits it into three parts to create a
    /// `Color` struct, or returns a default `Color` if the string does not have three parts.
    ///
    /// Arguments:
    ///
    /// * `value`: A string representing a color in the format "foreground:background:brightness".
    ///
    /// Returns:
    ///
    /// a `Color` struct.
    pub fn from_value(value: &str) -> Color {
        let split = value.split(':').collect::<Vec<&str>>();
        if split.len() == 3 {
            return Color {
                foreground: split.first().unwrap_or(&"").parse::<u8>().unwrap_or(0),
                background: split.get(1).unwrap_or(&"").parse::<u8>().unwrap_or(0),
                brightness: split.get(2).unwrap_or(&"").parse::<u8>().unwrap_or(0),
            };
        }
        Color::default()
    }
    /// The function `is_default` returns true if the `Color` struct is the default value.
    pub fn is_default(&self) -> bool {
        self.foreground == 0 && self.background == 0 && self.brightness == 0
    }
}
