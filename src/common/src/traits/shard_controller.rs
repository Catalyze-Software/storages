use std::fmt::Display;

use candid::Principal;
use catalyze_shared::{
    CanisterResult, CellStorage, Filter, GenericCellStorage, StaticCellStorageRef, StaticStorageRef,
};
use ic_stable_structures::Storable;

use crate::ShardStorage;

pub trait ShardController<K, V, F>
where
    K: candid::CandidType + 'static + Storable + Ord + Clone + Send + Sync,
    V: candid::CandidType + 'static + Storable + Clone + Send + Sync,
    F: candid::CandidType + Clone + Filter<K, V>,
{
    fn name(&self) -> String;

    fn storage_index(&self) -> StaticCellStorageRef<Principal>;

    fn storage_raw(&self) -> StaticStorageRef<K, V>;

    fn storage(&self) -> impl ShardStorage<K, V> {
        Storage::new(self.name(), self.storage_raw())
    }

    fn index(&self) -> impl CellStorage<Principal> {
        GenericCellStorage::new("index", self.storage_index())
    }

    fn size(&self) -> CanisterResult<u64> {
        Ok(self.storage().size())
    }

    fn get(&self, id: K) -> CanisterResult<(K, V)> {
        self.storage().get(id)
    }

    fn get_many(&self, keys: Vec<K>) -> CanisterResult<Vec<(K, V)>> {
        Ok(self.storage().get_many(keys)) // To keep same return type
    }

    fn get_all(&self) -> CanisterResult<Vec<(K, V)>> {
        Ok(self.storage().get_all())
    }

    fn filter_callback(&self, filters: Vec<F>) -> impl Fn(&K, &V) -> bool {
        move |id, event| filters.iter().all(|f| f.matches(id, event))
    }

    fn find(&self, filters: Vec<F>) -> CanisterResult<Option<(K, V)>> {
        Ok(self.storage().find(self.filter_callback(filters)))
    }

    fn filter(&self, filters: Vec<F>) -> CanisterResult<Vec<(K, V)>> {
        Ok(self.storage().filter(self.filter_callback(filters)))
    }

    fn insert(&self, key: K, value: V) -> CanisterResult<(K, V)> {
        self.storage().insert_by_key(key, value)
    }

    fn insert_many(&self, list: Vec<(K, V)>) -> CanisterResult<Vec<(K, V)>> {
        self.storage().insert_by_key_many(list)
    }

    fn update(&self, key: K, value: V) -> CanisterResult<(K, V)> {
        self.storage().update(key, value)
    }

    fn update_many(&self, list: Vec<(K, V)>) -> CanisterResult<Vec<(K, V)>> {
        self.storage().update_many(list)
    }

    fn remove(&self, key: K) -> CanisterResult<bool> {
        Ok(self.storage().remove(key))
    }

    fn remove_many(&self, keys: Vec<K>) -> CanisterResult<()> {
        self.storage().remove_many(keys);
        Ok(())
    }
}

pub struct Storage<K, V>
where
    K: candid::CandidType + 'static + Storable + Ord + Clone + Send + Sync,
    V: candid::CandidType + 'static + Storable + Clone + Send + Sync,
{
    pub name: String,
    pub raw: StaticStorageRef<K, V>,
}

impl<K, V> Storage<K, V>
where
    K: candid::CandidType + 'static + Storable + Ord + Clone + Send + Sync,
    V: candid::CandidType + 'static + Storable + Clone + Send + Sync,
{
    pub fn new<S: Display>(name: S, raw: StaticStorageRef<K, V>) -> Self {
        Self {
            name: name.to_string(),
            raw,
        }
    }
}

impl<K, V> ShardStorage<K, V> for Storage<K, V>
where
    K: candid::CandidType + 'static + Storable + Ord + Clone + Send + Sync,
    V: candid::CandidType + 'static + Storable + Clone + Send + Sync,
{
    fn name(&self) -> String {
        self.name.clone()
    }

    fn storage(&self) -> StaticStorageRef<K, V> {
        self.raw
    }
}
