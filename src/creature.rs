use slug::slugify;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[derive(Debug, Clone)]
pub struct Creature {
    identifier: String,
    parent_raw: String,
    #[serde(rename = "objectId")]
    object_id: String,
    pub name: String,
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
            name:  String::new(),
            description:  String::new(),
            object_id: format!("{}-{}-{}", raw, "CREATURE", slugify(id)),
            lays_eggs: false,
            egg_size: 0,
            clutch_size: [0,0],
            max_age: [0,0],
            based_on: String::new(),
        }
    }    
}