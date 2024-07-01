use crate::{
    metadata::RawObject,
    tags::{CasteTag, CreatureTag},
};

pub trait RawObjectToken<T: RawObject> {
    #[allow(dead_code)]
    fn is_within(&self, object: &T) -> bool;
}

impl RawObjectToken<creature::Creature> for CreatureTag {
    fn is_within(&self, object: &creature::Creature) -> bool {
        object.get_tags().contains(self)
    }
}

impl RawObjectToken<creature::Creature> for CasteTag {
    fn is_within(&self, object: &creature::Creature) -> bool {
        for caste in object.get_castes() {
            if caste.get_tags().contains(self) {
                return true;
            }
        }
        false
    }
}
