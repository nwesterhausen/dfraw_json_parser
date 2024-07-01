use crate::tags::PlantGraphicTemplateTag;

/// Map of plant graphic templates to their string representation.
pub static PLANT_GRAPHIC_TEMPLATE_TOKENS: phf::Map<&'static str, PlantGraphicTemplateTag> = phf::phf_map! {
  "STANDARD_LEAVES" => PlantGraphicTemplateTag::StandardLeaves,
  "STANDARD_FLOWERS_1" => PlantGraphicTemplateTag::StandardFlowers1,
  "STANDARD_FRUIT_1" => PlantGraphicTemplateTag::StandardFruit1,
  "STANDARD_FLOWERS_2" => PlantGraphicTemplateTag::StandardFlowers2,
  "STANDARD_FRUIT_2" => PlantGraphicTemplateTag::StandardFruit2,
  "STANDARD_FLOWERS_3" => PlantGraphicTemplateTag::StandardFlowers3,
  "STANDARD_FRUIT_3" => PlantGraphicTemplateTag::StandardFruit3,
  "STANDARD_FLOWERS_4" => PlantGraphicTemplateTag::StandardFlowers4,
  "STANDARD_FRUIT_4" => PlantGraphicTemplateTag::StandardFruit4,
};
