use std::fmt::Display;

use candid::Principal;
use catalyze_shared::StaticStorageRef;
use ic_stable_structures::Storable;

pub struct Registry<K>
where
    K: 'static
        + candid::CandidType
        + for<'a> candid::Deserialize<'a>
        + std::hash::Hash
        + Storable
        + Ord
        + Clone
        + Display
        + Send
        + Sync,
{
    name: String,
    storage: StaticStorageRef<K, Principal>,
}

impl<K> Registry<K>
where
    K: 'static
        + candid::CandidType
        + for<'a> candid::Deserialize<'a>
        + std::hash::Hash
        + Storable
        + Ord
        + Clone
        + Display
        + Send
        + Sync,
{
    pub fn new(storage: StaticStorageRef<K, Principal>) -> Self {
        Self {
            name: "registry".to_owned(),
            storage,
        }
    }
}

impl<K> crate::Registry<K> for Registry<K>
where
    K: 'static
        + candid::CandidType
        + for<'a> candid::Deserialize<'a>
        + std::hash::Hash
        + Storable
        + Ord
        + Clone
        + Display
        + Send
        + Sync,
{
    fn name(&self) -> String {
        self.name.clone()
    }

    fn storage(&self) -> StaticStorageRef<K, Principal> {
        self.storage
    }
}
