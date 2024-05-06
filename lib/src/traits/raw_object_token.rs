use crate::{creature, creature_caste, RawObject};

pub trait RawObjectToken<T: RawObject> {
    #[allow(dead_code)]
    fn is_within(&self, object: &T) -> bool;
}

impl RawObjectToken<creature::Creature> for creature::Token {
    fn is_within(&self, object: &creature::Creature) -> bool {
        object.get_tags().contains(self)
    }
}

impl RawObjectToken<creature::Creature> for creature_caste::Token {
    fn is_within(&self, object: &creature::Creature) -> bool {
        for caste in object.get_castes() {
            if caste.get_tags().contains(self) {
                return true;
            }
        }
        false
    }
}
