/// Trait for creature variation requirements
#[typetag::serde]
pub trait CreatureVariationRequirements {
    /// Remove a tag from the creature
    fn remove_tag(&mut self, key: &str);
    /// Remove a tag and value from the creature
    fn remove_tag_and_value(&mut self, key: &str, value: &str);
    /// Remove a tag from the creature caste
    fn remove_tag_for_caste(&mut self, key: &str, caste: &str);
    /// Remove a tag and value from the creature caste
    fn remove_tag_and_value_for_caste(&mut self, key: &str, value: &str, caste: &str);
    /// Add a tag to the creature
    fn add_tag(&mut self, key: &str);
    /// Add a tag and value to the creature
    fn add_tag_and_value(&mut self, key: &str, value: &str);
    /// Add a tag to the creature caste
    fn add_tag_for_caste(&mut self, key: &str, caste: &str);
    /// Add a tag and value to the creature caste
    fn add_tag_and_value_for_caste(&mut self, key: &str, value: &str, caste: &str);
}
