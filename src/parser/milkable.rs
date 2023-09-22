use serde::{Deserialize, Serialize};

#[allow(clippy::module_name_repetitions)]
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct DFMilkable {
    material: String,
    frequency: u32,
}
impl DFMilkable {
    pub(crate) fn from_value(value: &str) -> Self {
        let split = value.split(':').collect::<Vec<&str>>();
        if split.len() == 2 {
            return DFMilkable {
                material: String::from(split[0]),
                frequency: split[1].parse::<u32>().unwrap_or(0),
            };
        }
        DFMilkable::default()
    }
    pub fn is_default(&self) -> bool {
        DFMilkable::default().material.is_empty() && DFMilkable::default().frequency == 0
    }
}
