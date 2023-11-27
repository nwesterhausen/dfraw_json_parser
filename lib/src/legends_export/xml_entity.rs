use crate::parser::{entity::Entity, RawMetadata};

#[derive(Debug, Default)]
pub(crate) struct ExportedEntity {
    entity_id: u32,
    race: String,
    entity_type: String,
    child_id: u32,
    // population: u32,
    // civ_id: u32,
    // positions: Vec<ExportedEntityPosition>,
}

// #[derive(Debug, Default)]
// pub(crate) struct ExportedEntityPosition {
//     id: u32,
//     name: String,
//     male_name: String,
//     female_name: String,
//     spouse_name: String,
//     male_spouse_name: String,
//     female_spouse_name: String,
// }

impl ExportedEntity {
    pub fn set_id(&mut self, id: u32) {
        self.entity_id = id;
    }
    pub fn set_race(&mut self, race: &str) {
        self.race = race.into();
    }
    pub fn set_entity_type(&mut self, entity_type: &str) {
        self.entity_type = entity_type.into();
    }
    pub fn set_child_id(&mut self, child_id: u32) {
        self.child_id = child_id;
    }
    #[allow(dead_code)]
    pub fn into_entity(self, legend_metadata: &RawMetadata) -> Entity {
        Entity::new(
            format!("{}-{}{}", self.race, self.entity_type, self.entity_id).as_str(),
            legend_metadata,
        )
    }
    // pub fn set_population(&mut self, population: u32) {
    //     self.population = population;
    // }
    // pub fn set_civ_id(&mut self, civ_id: u32) {
    //     self.civ_id = civ_id;
    // }
}
