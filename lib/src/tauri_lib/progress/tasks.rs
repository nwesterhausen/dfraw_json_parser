use serde::{Deserialize, Serialize};

/// Tasks that describe what the parser is currently doing.

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
#[allow(clippy::module_name_repetitions)]
pub enum ProgressTask {
    /// Parsing a raw file
    ParseRaws,
    /// Parsing a legends file
    ParseLegends,
    /// Parsing a module `info.txt` file
    ParseModuleInfoFiles,
    /// Resolving raws.
    ///
    /// Creature raws (and others, but only creature raws are currently supported) can reference other creatures
    /// from the entire set of raws. This requires all the raws to be parsed before the references can be resolved.
    ///
    /// Eventually, this step will include other raw types, because it enables things like `CUT_X`, `SELECT_X`, etc.
    ResolveRaws,
    /// Sitting idle, probably sending a progress update or not yet started.
    ///
    /// This step also includes the time spent parsing the `ParserOptions` and validating them.
    #[default]
    Idle,
}
