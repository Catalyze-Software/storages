use candid::Principal;
use catalyze_shared::{
    attendee::Attendee, event_collection::EventCollection, StaticCellStorageRef, StaticStorageRef,
};
use common::{
    IndexConfig, IndexConfigBase, IndexConfigWithKeyIter, Principals, ShardStorage, ShardsIndex,
    Storage,
};

use crate::{aliases::Key, state::*};

#[derive(Clone)]
pub struct Config {
    proxies: StaticCellStorageRef<Principals>,
    shards: StaticCellStorageRef<ShardsIndex>,
    shard_iter: StaticCellStorageRef<Principal>,
    shard_wasm: StaticCellStorageRef<Vec<u8>>,
    registry: StaticStorageRef<Key, Principal>,
    key_iter: StaticCellStorageRef<Key>,

    attendees: StaticStorageRef<Principal, Attendee>,
    group_events: StaticStorageRef<u64, EventCollection>,
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

            attendees: &ATTENDEES,
            group_events: &GROUP_EVENTS,
        }
    }
}

impl IndexConfigBase<Key> for Config {
    fn storage_proxies(&self) -> StaticCellStorageRef<Principals> {
        self.proxies
    }
}

impl IndexConfig<Key> for Config {
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

impl Config {
    pub fn attendees(&self) -> impl ShardStorage<Principal, Attendee> {
        Storage::new("attendees", self.attendees)
    }

    pub fn group_events(&self) -> impl ShardStorage<u64, EventCollection> {
        Storage::new("group_events", self.group_events)
    }
}

pub fn config() -> Config {
    Config::default()
}
