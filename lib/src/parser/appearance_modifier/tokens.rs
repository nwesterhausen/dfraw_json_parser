pub enum AppearanceModifierToken {
    /// `APP_MOD_DESC_RANGE`
    ///
    /// If you use APP_MOD_DESC_RANGE, then the numbers that follow determine the number ranges where the appearance modifier is used.
    /// So if your range is 30:60:90:110:150:190, then the ranges of description are
    /// 'minimum'-30, 31-60, 61-90, 91-110, 111-150, 151-190, and 191-maximum
    /// (whatever your minimum and maximum are from the BP_APPEARANCE_MODIFIER ranges).
    DescriptionRange {
        lowest: i32,
        lowest: i32,
        low: i32,
        high: i32,
        higher: i32,
        highest: i32,
    },
    /// `APP_MOD_GENETIC_MODEL`
    GeneticModel { mode: GeneticModelMode },
    /// `APP_MOD_IMPORTANCE`
    Importance { importance: i32 },
    /// `APP_MOD_NOUN`
    Noun { noun: String, singular: bool },
    /// `APP_MOD_RATE`
    Rate {
        rate: i32,
        scale_daily: bool, // DAILY vs YEARLY
        min_growth: i32,
        max_growth: i32,
        start_year: i32,
        start_day: i32,
        no_end: bool,
        end_year: Option<i32>,
        end_day: Option<i32>,
    },
}

pub enum GeneticModelMode {
    DominantMore,
    DominantLess,
    Mix,
}
