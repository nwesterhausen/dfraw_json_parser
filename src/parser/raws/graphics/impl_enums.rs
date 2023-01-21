use super::{Color, Condition};

impl Condition {
    pub fn from_str(condition: &str) -> Self {
        match condition {
            "DEFAULT" => Condition::Default,
            "ANIMATED" => Condition::Animated,
            "CORPSE" => Condition::Corpse,
            "CHILD" => Condition::Child,
            "TRAINED_WAR" => Condition::TrainedWar,
            "TRAINED_HUNTER" => Condition::TrainedHunter,
            "LIST_ICON" => Condition::ListIcon,
            "SKELETON" => Condition::Skeleton,
            "SKELETON_WITH_SKULL" => Condition::SkeletonWithSkull,
            _ => Condition::None,
        }
    }
}

impl Color {
    pub fn from_str(_color: &str) -> Self {
        Color::AsIs
        // match color {
        //     "AS_IS" => Color::AsIs,
        //     _ => Color::AsIs,
        // }
    }
}
