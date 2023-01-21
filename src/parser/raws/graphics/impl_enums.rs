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
            "ZOMBIE" => Condition::Zombie,
            "NECROMANCER" => Condition::Necromancer,
            "MALE" => Condition::Male,
            "FEMALE" => Condition::Female,
            "VAMPCURSE" => Condition::VampireCursed,
            "GHOUL" => Condition::Ghoul,
            "DISTURBED_DEAD" => Condition::DisturbedDead,
            "NOT_ARTIFACT" => Condition::NotArtifact,
            "IS_CRAFTED_ARTIFACT" => Condition::CraftedArtifact,
            "CONDITION_DYE" => Condition::Dye,
            "CONDITION_NOT_DYED" => Condition::NotDyed,
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
