use serde::{Deserialize, Serialize};

#[derive(ts_rs::TS)]
#[ts(export)]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Default)]
pub enum SyndromeToken {
    /// Defines the name of the syndrome
    Name,
    /// Syndrome can be contracted by injection (by a creature)
    Injected,
    /// Syndrome can be contracted on contact (e.g. poison dust or liquid)
    Contact,
    /// Syndrome can be contracted by inhalation (e.g. poison vapor or gas)
    Inhaled,
    /// Syndrome can be contracted by ingestion (when the material is eaten in solid or liquid form)
    Ingested,
    /// Adds a class of creatures to those affected, such as `GENERAL_POISON` from \[CREATURE_CLASS:GENERAL_POISON\] tag.
    AffectedClass,
    /// Makes the class of creatures immune to the syndrome.
    ImmuneClass,
    /// Adds a specific creature to those affected. \[creature name:caste name || ALL\]
    AffectedCreature,
    /// Makes the creature immune to the syndrome. \[creature name:caste name || ALL\]
    ImmuneCreature,
    /// Unknown as default.
    #[default]
    Unknown,
    /// Seen the \[SYN_CONCENTRATION_ADDED:100:1000\] tag in material_templates.txt
    ConcentrationAdded,
    /// Seen the \[SYN_IDENTIFIER:INEBRIATION\] tag in material_templates.txt
    Identifier,
}
