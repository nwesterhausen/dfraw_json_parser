use serde::{Deserialize, Serialize};

#[derive(ts_rs::TS)]
#[ts(export)]
#[allow(clippy::module_name_repetitions)]
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct Color {
    foreground: u8,
    background: u8,
    brightness: u8,
}

impl Color {
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
    pub fn is_default(&self) -> bool {
        self.foreground == 0 && self.background == 0 && self.brightness == 0
    }
}
