use tracing::warn;

use crate::{
    creature::Creature,
    creature_variation::{CreatureVariation, Rule},
    ObjectType, RawObject,
};

pub fn apply_creature_variations(all_raws: &mut Vec<Box<dyn RawObject>>) {
    let mut creature_variations: Vec<CreatureVariation> = all_raws
        .iter()
        .filter(|r| r.get_type() == &ObjectType::CreatureVariation)
        .filter_map(|r| r.as_any().downcast_ref::<CreatureVariation>())
        .cloned()
        .collect();

    let mut creatures: Vec<Creature> = all_raws
        .iter()
        .filter(|r| r.get_type() == &ObjectType::Creature)
        .filter_map(|r| r.as_any().downcast_ref::<Creature>())
        .cloned()
        .collect();

    let mut updated_creatures: Vec<Creature> = Vec::new();

    // Go through all creatures and if they have a variation, apply it.
    for creature in creatures.iter_mut() {
        // Check variations against known creature variations
        for variation_id in creature.get_variations_to_apply() {
            let Some(creature_variation) = creature_variations
                .iter_mut()
                .find(|r| r.get_object_id() == variation_id)
            else {
                warn!(
                    "Failed to find creature variation {} for {}",
                    variation_id,
                    creature.get_object_id()
                );
                continue;
            };

            // Apply variation to creature
        }
    }
}

fn apply_variation_to_creature(creature: &mut Creature, creature_variation: &CreatureVariation) {
    // First filter all rules for convert rules to apply from bottom up
    let convert_rules: Vec<&Rule> = creature_variation
        .get_convert_rules()
        .iter()
        .rev()
        .cloned()
        .collect();
}
