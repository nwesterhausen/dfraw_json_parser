use crate::parser::object_types::ObjectType;

pub const PARSABLE_OBJECT_TYPES: [&ObjectType; 8] = [
    &ObjectType::Creature,
    &ObjectType::Plant,
    &ObjectType::Inorganic,
    &ObjectType::Graphics,
    &ObjectType::TilePage,
    &ObjectType::Entity,
    &ObjectType::MaterialTemplate,
    &ObjectType::CreatureVariation,
];
