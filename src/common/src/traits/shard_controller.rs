use candid::Principal;
use catalyze_shared::{CanisterResult, Filter, StaticStorageRef};
use ic_stable_structures::Storable;

use crate::ShardStorage;

use super::{CellStorage, StaticCellStorageRef};

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
        Index::new(self.storage_index())
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

struct Index {
    name: String,
    storage: StaticCellStorageRef<Principal>,
}

impl Index {
    pub fn new(storage: StaticCellStorageRef<Principal>) -> Self {
        Self {
            name: "index".to_owned(),
            storage,
        }
    }
}

impl CellStorage<Principal> for Index {
    fn name(&self) -> String {
        self.name.clone()
    }

    fn storage(&self) -> StaticCellStorageRef<Principal> {
        self.storage
    }
}

struct Storage<K, V>
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
    pub fn new(name: String, raw: StaticStorageRef<K, V>) -> Self {
        Self { name, raw }
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
