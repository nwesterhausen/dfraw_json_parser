use tracing::info;

use crate::{
    parser::{
        creature::raw::Creature, helpers::clone_raw_vector::with_purge, object_types::ObjectType,
        raws::RawObject, select_creature::raw::SelectCreature,
    },
    util::get_only_select_creatures_from_raws,
};

#[allow(clippy::too_many_lines)]
pub fn absorb_select_creature(all_raws: &mut Vec<Box<dyn RawObject>>) {
    let all_select_creatures = { get_only_select_creatures_from_raws(all_raws) };

    info!(
        "looking at {} SELECT_CREATURE of {} raws",
        all_select_creatures.len(),
        all_raws.len()
    );

    let mut object_ids_to_purge: Vec<&str> = Vec::new();
    let mut new_creatures: Vec<Creature> = Vec::new();
    let mut target_creature_identifiers: Vec<&str> = Vec::new();

    for select_creature in all_select_creatures.as_slice() {
        if target_creature_identifiers.contains(&select_creature.get_identifier()) {
            continue;
        }
        target_creature_identifiers.push(select_creature.get_identifier());
    }

    for raw in &*all_raws {
        if raw.get_type() == &ObjectType::Creature
            && target_creature_identifiers.contains(&raw.get_identifier())
        {
            let select_creature_vec: Vec<SelectCreature> = all_select_creatures
                .iter()
                .filter(|r| r.get_identifier() == raw.get_identifier())
                .cloned()
                .collect();

            if select_creature_vec.is_empty() {
                // Skip this creature if there are no select_creature records for it
                continue;
            }

            let mut temp_creature = raw
                .as_any()
                .downcast_ref::<Creature>()
                .unwrap_or(&Creature::empty())
                .clone();
            temp_creature.extend_select_creature_variation(select_creature_vec);

            let object_id = raw.get_object_id();
            object_ids_to_purge.push(object_id);

            if !temp_creature.is_empty() {
                new_creatures.push(temp_creature.clone());
            }
        }
    }

    if new_creatures.is_empty() {
        return;
    }

    object_ids_to_purge.extend(
        new_creatures
            .iter()
            .flat_map(Creature::get_child_object_ids)
            .collect::<Vec<&str>>(),
    );

    let mut new_raws: Vec<Box<dyn RawObject>> =
        with_purge(all_raws.as_slice(), object_ids_to_purge.as_slice());

    for creature in new_creatures {
        new_raws.push(Box::new(creature));
    }

    info!(
        "finished with {} raws (some {} purged)",
        new_raws.len(),
        all_raws.len() - new_raws.len()
    );

    *all_raws = new_raws;
}
