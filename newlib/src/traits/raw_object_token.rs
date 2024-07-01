use crate::{
    creature::Creature,
    tags::{CasteTag, CreatureTag},
};

use super::RawObject;

pub trait RawObjectToken<T: RawObject> {
    #[allow(dead_code)]
    fn is_within(&self, object: &T) -> bool;
}

impl RawObjectToken<Creature> for CreatureTag {
    fn is_within(&self, object: &Creature) -> bool {
        object.get_tags().contains(self)
    }
}

impl RawObjectToken<Creature> for CasteTag {
    fn is_within(&self, object: &Creature) -> bool {
        for caste in object.get_castes() {
            if caste.get_tags().contains(self) {
                return true;
            }
        }
        false
    }
}
