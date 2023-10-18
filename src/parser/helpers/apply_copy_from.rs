use itertools::Itertools;

use crate::parser::{
    creature::raw::Creature, helpers::clone_raw_vector::with_purge, object_types::ObjectType,
    raws::RawObject,
};

#[allow(clippy::too_many_lines)]
pub fn apply_copy_tags_from(all_raws: &mut Vec<Box<dyn RawObject>>) {
    let creatures_with_copy_tags_from: Vec<Creature> = {
        all_raws
            .iter()
            .filter(|r| r.get_type() == &ObjectType::Creature)
            .unique_by(|r| r.get_object_id())
            .map(|r| {
                r.as_any()
                    .downcast_ref::<Creature>()
                    .unwrap_or(&Creature::empty())
                    .clone()
            })
            .filter(|c| c.get_copy_tags_from() != "")
            .collect()
    };
    log::info!(
        "apply_copy_tags_from looking at {} of {} raws",
        creatures_with_copy_tags_from.len(),
        all_raws.len()
    );

    let mut target_creature_identifiers: Vec<&str> = Vec::new();
    let mut new_creatures: Vec<Creature> = Vec::new();

    for creature in creatures_with_copy_tags_from.as_slice() {
        if target_creature_identifiers.contains(&creature.get_copy_tags_from()) {
            continue;
        }
        target_creature_identifiers.push(creature.get_copy_tags_from());
    }

    for raw in &*all_raws {
        if raw.get_type() == &ObjectType::Creature
            && target_creature_identifiers.contains(&raw.get_identifier())
        {
            for creature in creatures_with_copy_tags_from.as_slice() {
                if creature.get_copy_tags_from() == raw.get_identifier() {
                    if new_creatures
                        .iter()
                        .map(RawObject::get_identifier)
                        .contains(&creature.get_identifier())
                    {
                        // Update the creature in new_creatures instead of adding a new one
                        let temp_creature = raw
                            .as_any()
                            .downcast_ref::<Creature>()
                            .unwrap_or(&Creature::empty())
                            .clone();
                        // Grab the creature from new_creatures
                        let mut new_creature = new_creatures
                            .iter()
                            .find(|c| c.get_identifier() == creature.get_identifier())
                            .unwrap_or(&Creature::empty())
                            .clone();
                        // Remove the creature from new_creatures
                        new_creatures.retain(|c| c.get_identifier() != creature.get_identifier());
                        // Apply copy tags from to new_creature
                        new_creature = Creature::copy_tags_from(&new_creature, &temp_creature);
                        // Add the updated creature back to new_creatures
                        new_creatures.push(new_creature);
                        continue;
                    }
                    let temp_creature = raw
                        .as_any()
                        .downcast_ref::<Creature>()
                        .unwrap_or(&Creature::empty())
                        .clone();

                    new_creatures.push(Creature::copy_tags_from(creature, &temp_creature));
                }
            }
        }
    }

    log::info!(
        "apply_copy_tags_from updated {} creatures",
        new_creatures.len()
    );

    let mut object_ids_to_purge: Vec<&str> = Vec::new();

    object_ids_to_purge.extend(
        new_creatures
            .iter()
            .map(RawObject::get_object_id)
            .collect::<Vec<&str>>(),
    );

    log::info!(
        "apply_copy_tags_from purging {} objects\n{:#?}",
        object_ids_to_purge.len(),
        object_ids_to_purge
    );

    let mut new_raws: Vec<Box<dyn RawObject>> =
        with_purge(all_raws, object_ids_to_purge.as_slice());

    for creature in new_creatures {
        new_raws.push(Box::new(creature));
    }

    if all_raws.len() < new_raws.len() {
        log::warn!(
            "apply_copy_tags_from finished with {} raws, but started with {}",
            new_raws.len(),
            all_raws.len()
        );
    } else {
        log::info!(
            "apply_copy_tags_from finished with {} raws (some {} purged)",
            new_raws.len(),
            all_raws.len() - new_raws.len()
        );
    }

    *all_raws = new_raws;
}
