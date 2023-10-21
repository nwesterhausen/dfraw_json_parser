use crate::parser::raws::RawObject;

use super::clone_raw_object_box::clone_raw_object_box;

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
