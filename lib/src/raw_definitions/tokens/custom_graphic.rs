use crate::tags::GraphicTypeTag;

/// Map of graphic type tags to their string representation.
pub static CUSTOM_GRAPHIC_TOKENS: phf::Map<&'static str, GraphicTypeTag> = phf::phf_map! {
  // [CUSTOM_EDGING:1] can be from 1 to 32, lower number is printed with higher priority,
  // all win out over regular grass
  "CUSTOM_EDGING" => GraphicTypeTag::CustomEdging,
  // [CUSTOM_RAMP:1] can be from 1 to 32, uses the plant's GRASS_1 image
  "CUSTOM_RAMP" => GraphicTypeTag::CustomRamp,
  // [CUSTOM_EDGE_W:CAVERN_GRASS:4:0]
  "CUSTOM_EDGE_W" => GraphicTypeTag::CustomEdgeW,
  // [CUSTOM_EDGE_E:CAVERN_GRASS:5:0]
  "CUSTOM_EDGE_E" => GraphicTypeTag::CustomEdgeE,
  // [CUSTOM_EDGE_S:CAVERN_GRASS:6:0]
  "CUSTOM_EDGE_S" => GraphicTypeTag::CustomEdgeS,
  // [CUSTOM_EDGE_N:CAVERN_GRASS:7:0]
  "CUSTOM_EDGE_N" => GraphicTypeTag::CustomEdgeN,
  // [CUSTOM_EDGE_NW:CAVERN_GRASS:8:0]
  "CUSTOM_EDGE_NW" => GraphicTypeTag::CustomEdgeNW,
  // [CUSTOM_EDGE_NE:CAVERN_GRASS:9:0]
  "CUSTOM_EDGE_NE" => GraphicTypeTag::CustomEdgeNE,
  // [CUSTOM_EDGE_SW:CAVERN_GRASS:10:0]
  "CUSTOM_EDGE_SW" => GraphicTypeTag::CustomEdgeSW,
  // [CUSTOM_EDGE_SE:CAVERN_GRASS:11:0]
  "CUSTOM_EDGE_SE" => GraphicTypeTag::CustomEdgeSE,
};
