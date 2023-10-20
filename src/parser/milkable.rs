use serde::{Deserialize, Serialize};

#[derive(ts_rs::TS)]
#[ts(export)]
#[allow(clippy::module_name_repetitions)]
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct Milkable {
    material: String,
    frequency: u32,
}
impl Milkable {
    pub fn from_value(value: &str) -> Self {
        let split = value.split(':').collect::<Vec<&str>>();
        if split.len() == 2 {
            let material_str = *split.first().unwrap_or(&"");
            return Milkable {
                material: String::from(material_str),
                frequency: split.get(1).unwrap_or(&"").parse::<u32>().unwrap_or(0),
            };
        }
        Milkable::default()
    }
    pub fn is_default(&self) -> bool {
        self.material.is_empty() && self.frequency == 0
    }
    pub fn as_vec(&self) -> Vec<String> {
        let mut vec = Vec::new();
        if !self.material.is_empty() {
            vec.push(self.material.clone());
        }
        if self.frequency > 0 {
            vec.push(self.frequency.to_string());
        }
        vec
    }
}
