use serde::{Deserialize, Serialize};
use tracing::debug;

use crate::parser::{
    creature::raw::Creature, creature_variation::raw::CreatureVariation, raws::RawObject,
};

use super::tokens::CVTag;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Default)]
struct CVNewTag {
    new_token_key: String,
    new_token_value: String,
}

#[typetag::serde]
impl CreatureVariation for CVNewTag {
    fn get_type(&self) -> CVTag {
        CVTag::NewTag
    }

    fn apply_tag(&self, creature: &mut Creature) {
        debug!(
            "Applying new tag {}:{} to creature {}",
            self.new_token_key,
            self.new_token_value,
            creature.get_identifier()
        );
        creature.parse_tag(&self.new_token_key, &self.new_token_value);
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Default)]
struct CVAddTag {
    add_token_key: String,
    add_token_value: String,
}

#[typetag::serde]
impl CreatureVariation for CVAddTag {
    fn get_type(&self) -> CVTag {
        CVTag::AddTag
    }

    fn apply_tag(&self, creature: &mut Creature) {
        debug!(
            "Applying add tag {}:{} to creature {}",
            self.add_token_key,
            self.add_token_value,
            creature.get_identifier()
        );
        creature.parse_tag(&self.add_token_key, &self.add_token_value);
    }
}
