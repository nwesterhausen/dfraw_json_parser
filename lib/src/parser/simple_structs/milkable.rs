use serde::{Deserialize, Serialize};

#[derive(ts_rs::TS)]
#[ts(export)]
#[allow(clippy::module_name_repetitions)]
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
/// How often a creature can be milked and what material it produces
pub struct Milkable {
    /// What material the milk is made up of.
    material: String,
    /// How often the creature can be milked.
    frequency: u32,
}

impl Milkable {
    /// Create a new `Milkable` struct from the value of a `MILKABLE` token.
    ///
    /// Arguments:
    ///
    /// * `value`: The value of the `MILKABLE` token, e.g. `LOCAL_CREATURE_MAT:MILK:20000` for `[MILKABLE:LOCAL_CREATURE_MAT:MILK:20000]`
    ///
    /// Returns:
    ///
    /// * `Milkable`: The newly created `Milkable` struct.
    pub fn from_value(value: &str) -> Self {
        let split = value.split(':').collect::<Vec<&str>>();
        // The only thing we know for sure, is that the last value will be the frequency.
        // The rest of the values should be joined together to form the material identifier.
        if split.len() > 1 {
            let frequency = split.last().unwrap_or(&"").parse::<u32>().unwrap_or(0);
            let material = split
                .iter()
                .take(split.len() - 1)
                .copied()
                .collect::<Vec<&str>>()
                .join(":");
            return Self {
                material,
                frequency,
            };
        }
        log::warn!("Unable to parse MILKABLE value: {}", value);
        // Return default if we can't parse the value.
        Self::default()
    }
    /// The function `is_default` returns true if the `Milkable` struct is the default value.
    pub fn is_default(&self) -> bool {
        self.material.is_empty() && self.frequency == 0
    }
    /// The function `as_vec` returns a vector of strings representing the `Milkable` struct.
    ///
    /// This is only used when creating a search string for the `Milkable` struct.
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
