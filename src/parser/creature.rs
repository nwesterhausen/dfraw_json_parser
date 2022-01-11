use super::raws::names;
use serde::{Deserialize, Serialize};
use slug::slugify;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Creature {
    identifier: String,
    parent_raw: String,
    #[serde(rename = "objectId")]
    object_id: String,
    pub names: Vec<names::Name>,
    pub child_names: Vec<names::ChildName>,
    pub lays_eggs: bool,
    pub egg_size: u32,
    pub clutch_size: [u32; 2],
    pub description: String,
    pub max_age: [u32; 2],
    pub based_on: String,
}

impl Creature {
    pub fn new(raw: &str, id: &str) -> Self {
        Self {
            identifier: String::from(id),
            parent_raw: String::from(raw),
            names: Vec::new(),
            child_names: Vec::new(),
            description: String::new(),
            object_id: format!("{}-{}-{}", raw, "CREATURE", slugify(id)),
            lays_eggs: false,
            egg_size: 0,
            clutch_size: [0, 0],
            max_age: [0, 0],
            based_on: String::new(),
        }
    }
}
