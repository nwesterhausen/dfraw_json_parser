//! String token to parsed tag map for tile page tokens.

use crate::tags::TilePageTag;

/// Map of tile page tags to their string representation.
pub static TILE_PAGE_TOKENS: phf::Map<&'static str, TilePageTag> = phf::phf_map! {
    "TILE_DIM" => TilePageTag::TileDim,
    "PAGE_DIM_PIXELS" => TilePageTag::PageDim,
    "PAGE_DIM" => TilePageTag::PageDim,
    "FILE" => TilePageTag::File,
};
