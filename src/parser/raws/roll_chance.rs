use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
/// Allows the ore to be smelted into metal in the smelter. Each token with a
/// non-zero chance causes the game to roll d100 four times, each time creating
/// one bar of the type requested on success.
pub struct RollChance {
    chance: u32,
    result: String,
}

impl RollChance {
    pub fn empty() -> Self {
        Self {
            chance: 0,
            result: String::new(),
        }
    }

    pub fn from_tag(tag_value: &str) -> Self {
        // Example COPPER:100
        let split = tag_value.split(':').collect::<Vec<&str>>();
        if split.len() != 2 {
            log::error!("Unable to parse metal and chance from {}", tag_value);
            return RollChance::empty();
        }

        let result = String::from(split[0]);

        match split[1].parse() {
            Ok(n) => return Self { chance: n, result },
            Err(e) => {
                log::warn!("Unable to parse chance from {},{:?}", split[1], e);
                return Self { chance: 0, result };
            }
        }
    }
}
