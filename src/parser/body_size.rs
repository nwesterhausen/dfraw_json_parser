use serde::{Deserialize, Serialize};

#[allow(clippy::module_name_repetitions)]
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
                years: split.first().unwrap_or(&"").parse::<u32>().unwrap_or(0),
                days: split.get(1).unwrap_or(&"").parse::<u32>().unwrap_or(0),
                size_cm3: split.get(2).unwrap_or(&"").parse::<u32>().unwrap_or(0),
            };
        }
        DFBodySize::default()
    }
}
