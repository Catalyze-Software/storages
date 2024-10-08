use candid::{CandidType, Deserialize, Principal};
use catalyze_shared::impl_storable_for;

use crate::Shard;

impl_storable_for!(ShardsIndex);

#[derive(Clone, Default, CandidType, Deserialize)]
pub struct ShardsIndex(Vec<Shard>);

impl ShardsIndex {
    pub fn new(list: Vec<Shard>) -> Self {
        Self(list)
    }

    pub fn to_vec(&self) -> Vec<Shard> {
        self.0.clone()
    }

    pub fn append(&mut self, other: &mut Vec<Shard>) {
        self.0.append(other);
    }

    pub fn contains(&self, shard: Principal) -> bool {
        self.0.iter().any(|s| s.id() == shard)
    }
}

impl From<Vec<Shard>> for ShardsIndex {
    fn from(list: Vec<Shard>) -> Self {
        Self::new(list)
    }
}
