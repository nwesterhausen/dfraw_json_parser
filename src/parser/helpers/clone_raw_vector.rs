use crate::parser::{
    creature::raw::Creature,
    graphics::{raw::Graphic, tile_page::TilePage},
    inorganic::raw::Inorganic,
    material_template::raw::MaterialTemplate,
    object_types::ObjectType,
    plant::raw::Plant,
    raws::RawObject,
    select_creature::raw::SelectCreature,
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
        _ => {
            log::warn!(
                "clone_raw_object_box has an unhandled object type: {:?}",
                box_ref.get_type()
            );
            Box::new(Creature::empty())
        }
    }
}

/// The function `clone_raw_vector_with_purge` clones a vector of raw objects, excluding those with
/// specified object IDs to purge.
///
/// Arguments:
///
/// * `all_raws`: A slice of `Box<dyn RawObject>`, which represents a collection of raw objects.
/// * `object_ids_to_purge`: A slice of string references representing the object IDs that need to be
/// purged from the `all_raws` vector.
///
/// Returns:
///
/// The function `clone_raw_vector_with_purge` returns a vector of boxed dynamic objects (`Vec<Box<dyn
/// RawObject>>`).
pub fn with_purge(
    all_raws: &[Box<dyn RawObject>],
    object_ids_to_purge: &[&str],
) -> Vec<Box<dyn RawObject>> {
    let mut new_raws: Vec<Box<dyn RawObject>> = Vec::new();

    for raw in all_raws {
        if object_ids_to_purge.contains(&raw.get_object_id()) {
            log::trace!("clone_raw_vector purging {}", raw.get_object_id());
        } else {
            // Match the object type, downcast and clone into a new box in new_raws
            new_raws.push(clone_raw_object_box(raw));
        }
    }
    new_raws
}

/// The function `with_limit_and_page` takes a slice of `RawObject` objects, a limit, and a page number,
/// and returns a new vector containing a subset of the original objects based on the limit and page
/// number.
///
/// Arguments:
///
/// * `all_raws`: A slice of boxed objects that implement the `RawObject` trait.
/// * `limit`: The `limit` parameter specifies the maximum number of items to be included in each page
/// of the result.
/// * `page`: The `page` parameter represents the page number of the data you want to retrieve. It is
/// used to calculate the starting and ending positions of the data based on the `limit` parameter. The
/// first page is represented by page number 1, so if you want to retrieve data from the first page
///
/// Returns:
///
/// a vector of boxed dynamic objects (`Vec<Box<dyn RawObject>>`).
pub fn with_limit_and_page(
    all_raws: &[Box<dyn RawObject>],
    limit: usize,
    page: usize,
) -> Vec<Box<dyn RawObject>> {
    let mut new_raws: Vec<Box<dyn RawObject>> = Vec::new();
    // Page 0 is the first page, so we need to subtract 1 from the page number
    let page = page - 1;
    let start = limit * page;
    let end = start + limit;

    log::debug!("with_limit_and_page start: {start}, end: {end}, page: {page}");

    for (pos, raw) in all_raws.iter().enumerate() {
        if pos >= start && pos < end {
            new_raws.push(clone_raw_object_box(raw));
        }
    }
    new_raws
}
