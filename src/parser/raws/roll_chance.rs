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
    pub fn from_tag(tag_value: &str) -> Option<Self> {
        // Example COPPER:100
        let split = tag_value.split(':').collect::<Vec<&str>>();
        if split.len() != 2 {
            log::error!("Unable to parse metal and chance from {}", tag_value);
            return None;
        }

        let Some(result) = split.first() else {
            log::warn!("Not enough pieces to tokenize in {}", tag_value);
            return None;
        };

        let Some(chance) = split.get(1) else {
            log::warn!("Not enough pieces to tokenize in {}", tag_value);
            return None;
        };

        match chance.parse() {
            Ok(n) => Some(Self {
                chance: n,
                result: String::from(*result),
            }),
            Err(e) => {
                log::warn!("Unable to parse chance from {},{:?}", chance, e);
                Some(Self {
                    chance: 0,
                    result: String::from(*result),
                })
            }
        }
    }
}
