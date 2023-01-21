use std::path::PathBuf;

use super::{dimensions::Dimensions, DFRawCommon};

mod impl_basic;
mod parse;
mod typed_json;

#[derive(Debug, Clone)]
#[allow(clippy::module_name_repetitions)]
pub struct DFTilePage {
    // Common Raw file Things
    raw_header: DFRawCommon,

    file: PathBuf,
    tile_dim: Dimensions,
    page_dim: Dimensions,
}
