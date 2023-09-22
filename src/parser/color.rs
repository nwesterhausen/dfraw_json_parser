use serde::{Deserialize, Serialize};

#[allow(clippy::module_name_repetitions)]
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct DFColor {
    foreground: u8,
    background: u8,
    brightness: u8,
}

impl DFColor {
    pub fn from_value(value: &str) -> DFColor {
        let split = value.split(':').collect::<Vec<&str>>();
        if split.len() == 3 {
            return DFColor {
                foreground: split.first().unwrap_or(&"").parse::<u8>().unwrap_or(0),
                background: split.get(1).unwrap_or(&"").parse::<u8>().unwrap_or(0),
                brightness: split.get(2).unwrap_or(&"").parse::<u8>().unwrap_or(0),
            };
        }
        DFColor::default()
    }
    pub fn is_default(&self) -> bool {
        self.foreground == 0 && self.background == 0 && self.brightness == 0
    }
}
