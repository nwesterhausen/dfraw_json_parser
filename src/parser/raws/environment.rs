use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum GroupingStyle {
    Vein,
    Custer,
    ClusterOne,
    ClusterSmall,
    None,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Environment {
    #[serde(rename = "surroundingRock")]
    surrounding_rock: String,
    grouping: GroupingStyle,
    frequency: u32,
}

impl Environment {
    pub fn empty() -> Self {
        Self {
            surrounding_rock: String::new(),
            grouping: GroupingStyle::None,
            frequency: 0,
        }
    }
    pub fn from_tag(tag_value: &str) -> Self {
        // Example: IGNEOUS_INTRUSIVE:CLUSTER_SMALL:100
        let split = tag_value.split(':').collect::<Vec<&str>>();
        if split.len() != 3 {
            log::error!(
                "Unable to parse rock_layer, grouping, and size from {}",
                tag_value
            );
            return Environment::empty();
        }

        let rock_layer = String::from(split[0]);

        let grouping = match *split.get(1).unwrap_or(&"") {
            "CLUSTER" => GroupingStyle::Custer,
            "CLUSTER_ONE" => GroupingStyle::ClusterOne,
            "CLUSTER_SMALL" => GroupingStyle::ClusterSmall,
            "VEIN" => GroupingStyle::Vein,
            _ => {
                log::info!("Unmatched environment grouping style: {}", &split[1]);
                GroupingStyle::None
            }
        };

        match split[2].parse() {
            Ok(n) => Self {
                grouping,
                surrounding_rock: rock_layer,
                frequency: n,
            },
            Err(e) => {
                log::warn!("Unable to parse size from {},{:?}", split[2], e);
                Self {
                    grouping,
                    surrounding_rock: rock_layer,
                    frequency: 0,
                }
            }
        }
    }
}
