use std::fmt::Display;

use catalyze_shared::{
    paged_response::PagedResponse, CanisterResult, Filter, Sorter, StaticStorageRef,
};
use ic_stable_structures::Storable;

use super::{ShardStorage, Storage};

pub trait IndexControllerStateful<K, V, F, S>: Send + Sync
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
    V: 'static
        + candid::CandidType
        + for<'a> candid::Deserialize<'a>
        + Storable
        + Clone
        + Send
        + Sync,
    F: 'static + Filter<K, V> + candid::CandidType + Clone + Send + Sync,
    S: 'static + Sorter<K, V> + Default + candid::CandidType + Clone + Send + Sync,
{
    fn name(&self) -> String;
    fn raw(&self) -> StaticStorageRef<K, V>;

    fn filter_callback(&self, filters: Vec<F>) -> impl Fn(&K, &V) -> bool {
        move |id, event| filters.iter().all(|f| f.matches(id, event))
    }

    fn storage(&self) -> impl ShardStorage<K, V> {
        Storage::new(self.name(), self.raw())
    }

    fn size(&self) -> CanisterResult<u64> {
        Ok(self.storage().size())
    }

    fn get(&self, id: K) -> CanisterResult<(K, V)> {
        self.storage().get(id)
    }

    fn get_many(&self, ids: Vec<K>) -> CanisterResult<Vec<(K, V)>> {
        Ok(self.storage().get_many(ids))
    }

    fn get_all(&self) -> CanisterResult<Vec<(K, V)>> {
        let res = self.storage().get_all();
        Ok(S::default().sort(res))
    }

    fn get_paginated(
        &self,
        limit: usize,
        page: usize,
        sort: S,
    ) -> CanisterResult<PagedResponse<(K, V)>> {
        let res = self.storage().get_all();
        Ok(PagedResponse::new(page, limit, sort.sort(res)))
    }

    fn find(&self, filters: Vec<F>) -> CanisterResult<Option<(K, V)>> {
        Ok(self.storage().find(self.filter_callback(filters)))
    }

    fn filter(&self, filters: Vec<F>) -> CanisterResult<Vec<(K, V)>> {
        Ok(self.storage().filter(self.filter_callback(filters)))
    }

    fn filter_paginated(
        &self,
        limit: usize,
        page: usize,
        sort: S,
        filters: Vec<F>,
    ) -> CanisterResult<PagedResponse<(K, V)>> {
        let res = self.storage().filter(self.filter_callback(filters));
        Ok(PagedResponse::new(page, limit, sort.sort(res)))
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
