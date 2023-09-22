use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct DFBodySize {
    years: u32,
    days: u32,
    size_cm3: u32,
}
impl DFBodySize {
    pub(crate) fn from_value(value: &str) -> DFBodySize {
        let split = value.split(':').collect::<Vec<&str>>();
        if split.len() == 3 {
            return DFBodySize {
                years: split[0].parse::<u32>().unwrap_or(0),
                days: split[1].parse::<u32>().unwrap_or(0),
                size_cm3: split[2].parse::<u32>().unwrap_or(0),
            };
        }
        DFBodySize::default()
    }
}
