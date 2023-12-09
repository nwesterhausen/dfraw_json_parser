use tracing::{debug, warn};

use crate::{creature::Creature, creature_variation::CreatureVariation, ObjectType, RawObject};

pub fn apply_creature_variations(all_raws: &mut [Box<dyn RawObject>]) {
    let creature_variations: Vec<CreatureVariation> = all_raws
        .iter()
        .filter(|r| r.get_type() == &ObjectType::CreatureVariation)
        .filter_map(|r| r.as_any().downcast_ref::<CreatureVariation>())
        .cloned()
        .collect();

    let creatures: Vec<Creature> = all_raws
        .iter()
        .filter(|r| r.get_type() == &ObjectType::Creature)
        .filter_map(|r| r.as_any().downcast_ref::<Creature>())
        .cloned()
        .collect();

    let mut updated_creatures: Vec<Creature> = Vec::new();

    // Go through all creatures and if they have a variation, apply it.
    for creature in creatures {
        // Check variations against known creature variations
        for variation in creature.get_variations_to_apply() {
            if let Some(updated_creature) = singularly_apply_creature_variation(
                &creature,
                variation,
                creature_variations.as_slice(),
            ) {
                updated_creatures.push(updated_creature);
            }
        }
    }

    // Replace creatures with updated creatures
    for updated_creature in updated_creatures {
        let Some(index) = all_raws
            .iter()
            .position(|r| r.get_object_id() == updated_creature.get_object_id())
        else {
            warn!(
                "Failed to find creature {} to replace with updated creature",
                updated_creature.get_object_id()
            );
            continue;
        };

        #[allow(clippy::indexing_slicing)]
        let _ = std::mem::replace(&mut all_raws[index], Box::new(updated_creature));
    }
}

pub fn singularly_apply_creature_variation(
    creature: &Creature,
    variation: &str,
    creature_variations: &[CreatureVariation],
) -> Option<Creature> {
    // The variation comes back like this:
    // "STANDARD_WALK_CRAWL_GAITS:6561:6115:5683:1755:7456:8567"
    // We need to split it into the variation id and the args (if any)
    let variation_parts: Vec<&str> = variation.split(':').collect();
    let variation_identifier = *variation_parts.first().unwrap_or(&"");
    let variation_args = variation_parts.get(1..).unwrap_or(&[]);

    let Some(creature_variation) = creature_variations
        .iter()
        .find(|r| r.get_identifier().to_uppercase() == variation_identifier.to_uppercase())
        .cloned()
    else {
        warn!("Failed to find creature variation {}", variation_identifier);
        debug!("args: {:?}", variation_args);
        return None;
    };

    let mut updated_creature = creature.clone();
    debug!(
        "Applying variation {} to {}",
        variation_identifier,
        creature.get_identifier()
    );

    // Reset to `ALL` caste; some variations contain caste-specific rules but do not specify a caste
    updated_creature.select_caste("ALL");

    // Apply variation to creature
    for rule in creature_variation.get_rules() {
        rule.apply(&mut updated_creature, variation_args);
    }

    Some(updated_creature)
}
