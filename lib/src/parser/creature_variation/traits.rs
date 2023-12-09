#[typetag::serde]
pub trait CreatureVariationRequirements {
    fn remove_tag(&mut self, key: &str);
    fn remove_tag_and_value(&mut self, key: &str, value: &str);
    fn remove_tag_for_caste(&mut self, key: &str, caste: &str);
    fn remove_tag_and_value_for_caste(&mut self, key: &str, value: &str, caste: &str);

    fn add_tag(&mut self, key: &str);
    fn add_tag_and_value(&mut self, key: &str, value: &str);
    fn add_tag_for_caste(&mut self, key: &str, caste: &str);
    fn add_tag_and_value_for_caste(&mut self, key: &str, value: &str, caste: &str);
}
