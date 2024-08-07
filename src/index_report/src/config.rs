use candid::Principal;
use catalyze_shared::StaticStorageRef;
use common::{IndexConfig, IndexConfigWithKeyIter, Principals, ShardsIndex, StaticCellStorageRef};

use crate::{aliases::Key, state::*};

#[derive(Clone)]
pub struct Config {
    proxies: StaticCellStorageRef<Principals>,
    shards: StaticCellStorageRef<ShardsIndex>,
    shard_iter: StaticCellStorageRef<Principal>,
    shard_wasm: StaticCellStorageRef<Vec<u8>>,
    registry: StaticStorageRef<Key, Principal>,
    key_iter: StaticCellStorageRef<Key>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            proxies: &PROXIES,
            shards: &SHARDS,
            shard_iter: &SHARD_ITER,
            shard_wasm: &SHARD_WASM,
            registry: &REGISTRY,
            key_iter: &KEY_ITER,
        }
    }
}

impl IndexConfig<Key> for Config {
    fn storage_proxies(&self) -> StaticCellStorageRef<Principals> {
        self.proxies
    }

    fn storage_shards(&self) -> StaticCellStorageRef<ShardsIndex> {
        self.shards
    }

    fn storage_shard_iter(&self) -> StaticCellStorageRef<Principal> {
        self.shard_iter
    }

    fn storage_shard_wasm(&self) -> StaticCellStorageRef<Vec<u8>> {
        self.shard_wasm
    }

    fn storage_registry(&self) -> StaticStorageRef<Key, Principal> {
        self.registry
    }
}

impl IndexConfigWithKeyIter for Config {
    fn storage_key_iter(&self) -> StaticCellStorageRef<Key> {
        self.key_iter
    }
}

pub fn config() -> Config {
    Config::default()
}
