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
            let material_str = *split.first().unwrap_or(&"");
            return DFMilkable {
                material: String::from(material_str),
                frequency: split.get(1).unwrap_or(&"").parse::<u32>().unwrap_or(0),
            };
        }
        DFMilkable::default()
    }
    pub fn is_default(&self) -> bool {
        self.material.is_empty() && self.frequency == 0
    }
}
