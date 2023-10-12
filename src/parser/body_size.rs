use serde::{Deserialize, Serialize};

#[derive(ts_rs::TS)]
#[ts(export)]
#[allow(clippy::module_name_repetitions)]
#[derive(Serialize, Deserialize, Debug, Clone, Default, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct BodySize {
    years: u32,
    days: u32,
    size_cm3: u32,
}
impl BodySize {
    pub(crate) fn from_value(value: &str) -> BodySize {
        let split = value.split(':').collect::<Vec<&str>>();
        if split.len() == 3 {
            return BodySize {
                years: split.first().unwrap_or(&"").parse::<u32>().unwrap_or(0),
                days: split.get(1).unwrap_or(&"").parse::<u32>().unwrap_or(0),
                size_cm3: split.get(2).unwrap_or(&"").parse::<u32>().unwrap_or(0),
            };
        }
        BodySize::default()
    }
}
