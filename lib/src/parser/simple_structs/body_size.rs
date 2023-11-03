use serde::{Deserialize, Serialize};

#[derive(ts_rs::TS)]
#[ts(export)]
#[allow(clippy::module_name_repetitions)]
#[derive(Serialize, Deserialize, Debug, Clone, Default, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct BodySize {
    /// The number of years old the body is.
    years: u32,
    /// The number of days old the body is. (Days + years is total age.)
    days: u32,
    /// The volume of the body in cubic centimeters.
    size_cm3: u32,
}

impl BodySize {
    /// Create a new `BodySize` by parsing a `BODY_SIZE` token from the raws.
    ///
    /// Arguments:
    ///
    /// * `value`: The value of the `BODY_SIZE` token, e.g. `0:0:6000` for `[BODY_SIZE:0:0:6000]`
    pub fn from_value(value: &str) -> BodySize {
        let split = value.split(':').collect::<Vec<&str>>();
        if split.len() == 3 {
            return BodySize {
                years: split.first().unwrap_or(&"").parse::<u32>().unwrap_or(0),
                days: split.get(1).unwrap_or(&"").parse::<u32>().unwrap_or(0),
                size_cm3: split.get(2).unwrap_or(&"").parse::<u32>().unwrap_or(0),
            };
        }
        BodySize::default()
    }
}
