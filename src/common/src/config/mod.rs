use std::fmt::Display;

use candid::Principal;
use catalyze_shared::{CellStorage, GenericCellStorage, StaticCellStorageRef, StaticStorageRef};
use ic_stable_structures::Storable;
use key_iter::KeyIter;

use crate::{IDIter, Principals, ShardsIndex};

mod key_iter;
mod registry;

pub trait IndexConfigBase<K>: Send + Sync
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
    // storage API
    fn storage_proxies(&self) -> StaticCellStorageRef<Principals>;

    // public API
    fn proxies(&self) -> impl CellStorage<Principals> {
        GenericCellStorage::new("proxies", self.storage_proxies())
    }
}

pub trait IndexConfig<K>: IndexConfigBase<K>
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
    // storage API
    fn storage_shards(&self) -> StaticCellStorageRef<ShardsIndex>;
    fn storage_shard_iter(&self) -> StaticCellStorageRef<Principal>;
    fn storage_shard_wasm(&self) -> StaticCellStorageRef<Vec<u8>>;

    fn storage_registry(&self) -> StaticStorageRef<K, Principal>;

    // public API
    fn shards(&self) -> impl CellStorage<ShardsIndex> {
        GenericCellStorage::new("shards", self.storage_shards())
    }

    fn shard_iter(&self) -> impl CellStorage<Principal> {
        GenericCellStorage::new("shard_iter", self.storage_shard_iter())
    }

    fn shard_wasm(&self) -> impl CellStorage<Vec<u8>> {
        GenericCellStorage::new("shard_wasm", self.storage_shard_wasm())
    }

    fn registry(&self) -> impl crate::Registry<K> {
        registry::Registry::new(self.storage_registry())
    }
}

pub trait IndexConfigWithKeyIter: IndexConfigBase<u64> {
    fn storage_key_iter(&self) -> StaticCellStorageRef<u64>;

    fn key_iter(&self) -> impl IDIter {
        KeyIter::new(self.storage_key_iter())
    }
}
