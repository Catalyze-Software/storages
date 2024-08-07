use std::fmt::Display;

use candid::Principal;
use catalyze_shared::StaticStorageRef;
use ic_stable_structures::Storable;
use key_iter::KeyIter;
use proxies::Proxies;
use shard_iter::ShardIter;
use shard_wasm::ShardWasm;
use shards::Shards;

use crate::{CellStorage, IDIter, Principals, ShardsIndex, StaticCellStorageRef};

mod key_iter;
mod proxies;
mod registry;
mod shard_iter;
mod shard_wasm;
mod shards;

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
        Proxies::new(self.storage_proxies())
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
        Shards::new(self.storage_shards())
    }

    fn shard_iter(&self) -> impl CellStorage<Principal> {
        ShardIter::new(self.storage_shard_iter())
    }

    fn shard_wasm(&self) -> impl CellStorage<Vec<u8>> {
        ShardWasm::new(self.storage_shard_wasm())
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
