use crate::parser::raws::{
    names::{Name, SingPlurName},
    tags,
};

impl super::DFCreatureCaste {
    pub fn new(name: &str) -> Self {
        Self {
            // Identification
            name: String::from(name),
            // Boolean Flags
            tags: Vec::new(),

            // [min, max] ranges
            clutch_size: [0, 0],
            litter_size: [0, 0],
            max_age: [0, 0],

            // Combo flags (custom)
            active_time: 0, // MATUTINAL/DIURNAL/NOCTURNAL/CREPUSCULAR/VESPERTINE via binary math
            curious_beast: 0, // EATER/GUZZLER/ITEM via binary math
            no_season: 0,   // NO_SPRING/NO_SUMMER/NO_AUTUMN/NO_WINTER
            trainable: 0,   // trainable_HUNTING/trainable_WAR/BOTH(aka trainable)

            // Integer tokens
            baby: 0,
            child: 0,
            difficulty: 0,
            egg_size: 0,
            grass_trample: 0,
            grazer: 0,
            low_light_vision: 0,
            pet_value: 0,
            pop_ratio: 0,

            // String Tokens
            baby_name: SingPlurName::new(""),
            caste_name: Name::new(""),
            child_name: SingPlurName::new(""),
            description: String::new(),

            // Arrays
            creature_class: Vec::new(),

            // Custom tokens
            body_size: Vec::new(),
            milkable: tags::DFMilkable::new("", 0),
        }
    }
}
