use serde::{Deserialize, Serialize};

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
                foreground: split[0].parse::<u8>().unwrap_or(0),
                background: split[1].parse::<u8>().unwrap_or(0),
                brightness: split[2].parse::<u8>().unwrap_or(0),
            };
        }
        DFColor::default()
    }
    pub fn is_default(&self) -> bool {
        DFColor::default().foreground == 0
            && DFColor::default().background == 0
            && DFColor::default().brightness == 0
    }
}
