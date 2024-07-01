use super::RawObject;

#[typetag::serialize]
pub trait RawObjectToken<T: RawObject> {
    fn is_within(&self, object: &T) -> bool;
}
