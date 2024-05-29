use super::*;
#[derive(Clone, Debug)]
pub struct IdentifiedView<V> {
    pub id: u64,
    pub value: V,
}

impl<V> IdentifiedView<V> {
    pub fn new<ID: Hash>(id: ID, value: V) -> Self {
        Self {
            id: do_hash(&id),
            value,
        }
    }
}
