use slug::slugify;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[derive(Debug, Clone)]
pub struct Creature {
    identifier: String,
    parent_raw: String,
    objectId: String,
    pub name: String,
    pub lays_eggs: bool,
    pub egg_size: u32,
    pub clutch_size: [u32; 2],
    pub description: String,
    pub max_age: [u32; 2],
}

impl Creature {
    pub fn new(raw: &str, id: &str) -> Self {
        Self {
            identifier: String::from(id),
            parent_raw: String::from(raw),
            name: String::from("unknown"),
            description: String::from("unknown"),
            objectId: format!("{}-{}-{}", raw, "CREATURE", slugify(id)),
            lays_eggs: false,
            egg_size: 0,
            clutch_size: [0,0],
            max_age: [0,0],
        }
    }    
}