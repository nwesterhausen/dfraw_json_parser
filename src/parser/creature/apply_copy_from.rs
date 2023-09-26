use crate::parser::{object_types::ObjectType, raws::RawObject};

use super::raw::DFCreature;

pub fn apply_copy_tags_from(all_raws: &mut Vec<Box<dyn RawObject>>) {
    let all_creatures = all_raws
        .iter()
        .filter(|r| r.get_type() == &ObjectType::Creature)
        .map(|r| r.as_any().downcast_ref::<DFCreature>().unwrap().clone())
        .collect::<Vec<_>>();
    let mut creatures_to_replace: Vec<DFCreature> = Vec::new();

    for raw in &mut *all_raws {
        if raw.get_type() != &ObjectType::Creature {
            continue;
        }

        let creature = raw.as_any().downcast_ref::<DFCreature>().unwrap();

        if !creature.get_copy_tags_from().is_empty() {
            let target_creature_identifier = creature.get_copy_tags_from();
            // Get the creature to copy tags from
            let target_creature = all_creatures
                .iter()
                .find(|c| c.get_identifier() == target_creature_identifier)
                .cloned();

            if let Some(target_creature) = target_creature {
                log::debug!(
                    "Applying copy tags from {} to {}",
                    target_creature_identifier,
                    creature.get_identifier()
                );
                let new_creature = DFCreature::copy_tags_from(creature, &target_creature);
                creatures_to_replace.push(new_creature);
            } else {
                log::warn!(
                    "Could not find creature {} to copy tags from",
                    target_creature_identifier
                );
            }
        }
    }

    for creature in creatures_to_replace {
        // Remove the old creature from all_raws by matching creature identifier
        let old_creature_index = all_raws
            .iter()
            .position(|r| r.get_identifier() == creature.get_identifier());
        if let Some(old_creature_index) = old_creature_index {
            all_raws.remove(old_creature_index);
        }

        // Add replacement creature to all_raws
        all_raws.push(Box::new(creature));
    }
}
