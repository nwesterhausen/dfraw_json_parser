use itertools::Itertools;

use crate::parser::{
    creature::Creature,
    helpers::{clone_raw_object_box, clone_raw_vector::with_purge},
    object_type::ObjectType,
    raws::RawObject,
};

#[allow(clippy::too_many_lines)]
pub fn apply_copy_tags_from(all_raws: &mut Vec<Box<dyn RawObject>>) {
    let untouched_raws: Vec<_> = all_raws
        .iter()
        .map(clone_raw_object_box::clone_raw_object_box)
        .collect();

    let creatures_with_copy_tags_from: Vec<Creature> = {
        untouched_raws
            .iter()
            .filter(|r| r.get_type() == &ObjectType::Creature)
            .filter_map(|r| {
                let creature = r
                    .as_any()
                    .downcast_ref::<Creature>()
                    .unwrap_or(&Creature::empty())
                    .clone();

                if creature.get_copy_tags_from() == "" {
                    None
                } else {
                    Some(creature)
                }
            })
            .collect()
    };
    let source_creature_identifier_list: Vec<String> = creatures_with_copy_tags_from
        .iter()
        .map(|c| c.get_copy_tags_from().to_lowercase())
        .unique()
        .collect();
    log::info!(
        "apply_copy_tags_from: updating {} of {} raws from {} creatures",
        creatures_with_copy_tags_from.len(),
        all_raws.len(),
        source_creature_identifier_list.len()
    );

    // Build a list of unique creature identifiers to target, based on the apply_copy_tags_from list.
    let source_creatures: Vec<Creature> = untouched_raws
        .iter()
        .filter_map(|raw| {
            if raw.get_type() == &ObjectType::Creature
                && source_creature_identifier_list.contains(&raw.get_identifier().to_lowercase())
            {
                Some(
                    raw.as_any()
                        .downcast_ref::<Creature>()
                        .unwrap_or(&Creature::empty())
                        .clone(),
                )
            } else {
                None
            }
        })
        .collect::<Vec<Creature>>();

    // The outside loop iterates over the source creatures, which we will use to copy tags from
    // Inside the loop, we find which creatures have the source creature's identifier in their
    // copy_tags_from field, and then apply the source creature's tags to those creatures.
    // Then we put the updated creatures into the new_creatures vector, which will be used to
    // replace the old creatures in the all_raws vector.

    let mut new_creatures: Vec<Creature> = Vec::new();
    for source_creature in source_creatures {
        let target_creatures: Vec<Creature> = creatures_with_copy_tags_from
            .iter()
            .filter(|c| {
                c.get_copy_tags_from().to_lowercase()
                    == source_creature.get_identifier().to_lowercase()
            })
            .cloned()
            .collect::<Vec<Creature>>();

        for target_creature in target_creatures {
            new_creatures.push(Creature::copy_tags_from(&target_creature, &source_creature));
        }
    }

    log::info!(
        "apply_copy_tags_from: copied tags to {} creatures",
        new_creatures.len()
    );

    let mut object_ids_to_purge: Vec<&str> = Vec::new();

    object_ids_to_purge.extend(
        new_creatures
            .iter()
            .map(RawObject::get_object_id)
            .collect::<Vec<&str>>(),
    );

    let mut new_raws: Vec<Box<dyn RawObject>> =
        with_purge(all_raws, object_ids_to_purge.as_slice());

    if all_raws.len() < new_raws.len() {
        log::warn!(
            "apply_copy_tags_from: post purge has {} raws, but started with {}",
            new_raws.len(),
            all_raws.len()
        );
    } else {
        log::info!(
            "apply_copy_tags_from: purged {} raws",
            all_raws.len() - new_raws.len()
        );
    }

    for creature in new_creatures {
        new_raws.push(Box::new(creature));
    }

    if all_raws.len() < new_raws.len() {
        log::warn!(
            "apply_copy_tags_from: finished with {} raws, but started with {}",
            new_raws.len(),
            all_raws.len()
        );
    } else {
        log::info!(
            "apply_copy_tags_from: finished with {} raws (net {} lost)",
            new_raws.len(),
            all_raws.len() - new_raws.len()
        );
    }

    *all_raws = new_raws;
}
