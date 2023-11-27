use crate::parser::{creature::Creature, Name, RawMetadata};

#[derive(Debug, Default)]
pub(crate) struct ExportedCreature {
    creature_id: String,
    name_singular: String,
    name_plural: String,
    tags: Vec<String>,
}

impl ExportedCreature {
    pub fn is_empty(&self) -> bool {
        self.creature_id.is_empty()
    }
    pub fn set_creature_id(&mut self, creature_id: &str) {
        self.creature_id = creature_id.to_string();
    }
    pub fn set_name_singular(&mut self, name_singular: &str) {
        self.name_singular = name_singular.to_string();
    }
    pub fn set_name_plural(&mut self, name_plural: &str) {
        self.name_plural = name_plural.to_string();
    }
    pub fn add_tag(&mut self, tag: &str) {
        self.tags.push(tag.to_string());
    }
    pub fn into_creature(self, metadata: &RawMetadata) -> Creature {
        let mut creature = Creature::new(&self.creature_id, metadata);
        creature.set_name(Name::new(&self.name_singular, &self.name_plural, ""));
        creature.parse_tags_from_xml(self.tags.as_slice());

        creature
    }
}
