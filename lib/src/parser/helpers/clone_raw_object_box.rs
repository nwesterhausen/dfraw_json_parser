use tracing::warn;

use crate::{
    creature_variation::CreatureVariation,
    parser::{
        creature::Creature,
        entity::Entity,
        graphics::{Graphic, TilePage},
        inorganic::Inorganic,
        material_template::MaterialTemplate,
        plant::Plant,
        select_creature::SelectCreature,
        ObjectType, RawObject,
    },
};

#[allow(clippy::borrowed_box)]
/// The function `clone_raw_object_box` clones a boxed object based on its type.
///
/// Arguments:
///
/// * `box_ref`: A reference to a boxed object implementing the `RawObject` trait.
///
/// Returns:
///
/// The function `clone_raw_object_box` returns a `Box<dyn RawObject>`.
pub fn clone_raw_object_box(box_ref: &Box<dyn RawObject>) -> Box<dyn RawObject> {
    match box_ref.get_type() {
        ObjectType::Creature => {
            let temp_creature = box_ref
                .as_any()
                .downcast_ref::<Creature>()
                .unwrap_or(&Creature::empty())
                .clone();
            Box::new(temp_creature)
        }
        ObjectType::SelectCreature => {
            let temp_select_creature = box_ref
                .as_any()
                .downcast_ref::<SelectCreature>()
                .unwrap_or(&SelectCreature::empty())
                .clone();
            Box::new(temp_select_creature)
        }
        ObjectType::CreatureVariation => {
            let temp_creature_variation = box_ref
                .as_any()
                .downcast_ref::<CreatureVariation>()
                .unwrap_or(&CreatureVariation::empty())
                .clone();
            Box::new(temp_creature_variation)
        }
        ObjectType::Plant => {
            let temp_plant = box_ref
                .as_any()
                .downcast_ref::<Plant>()
                .unwrap_or(&Plant::empty())
                .clone();
            Box::new(temp_plant)
        }
        ObjectType::Inorganic => {
            let temp_inorganic = box_ref
                .as_any()
                .downcast_ref::<Inorganic>()
                .unwrap_or(&Inorganic::empty())
                .clone();
            Box::new(temp_inorganic)
        }
        ObjectType::MaterialTemplate => {
            let temp_material_template = box_ref
                .as_any()
                .downcast_ref::<MaterialTemplate>()
                .unwrap_or(&MaterialTemplate::empty())
                .clone();
            Box::new(temp_material_template)
        }
        ObjectType::Graphics => {
            let temp_graphic = box_ref
                .as_any()
                .downcast_ref::<Graphic>()
                .unwrap_or(&Graphic::empty())
                .clone();
            Box::new(temp_graphic)
        }
        ObjectType::TilePage => {
            let temp_tile_page = box_ref
                .as_any()
                .downcast_ref::<TilePage>()
                .unwrap_or(&TilePage::empty())
                .clone();
            Box::new(temp_tile_page)
        }
        ObjectType::Entity => {
            let temp_entity = box_ref
                .as_any()
                .downcast_ref::<Entity>()
                .unwrap_or(&Entity::empty())
                .clone();
            Box::new(temp_entity)
        }
        _ => {
            warn!(
                "clone_raw_object_box has an unhandled object type: {:?}",
                box_ref.get_type()
            );
            Box::new(Creature::empty())
        }
    }
}
