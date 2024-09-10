use candid::Principal;
use catalyze_shared::{
    state::{init_btree, init_cell, init_memory_manager},
    CellStorageRef, MemoryManagerStorage, StorageRef,
};
use common::{Principals, ShardsIndex};
use ic_stable_structures::memory_manager::MemoryId;

use crate::aliases::Key;

pub static PROXIES_MEMORY_ID: MemoryId = MemoryId::new(0);
pub static SHARDS_MEMORY_ID: MemoryId = MemoryId::new(1);
pub static KEY_ITER_MEMORY_ID: MemoryId = MemoryId::new(2);
pub static SHARD_ITER_MEMORY_ID: MemoryId = MemoryId::new(3);
pub static SHARD_WASM_MEMORY_ID: MemoryId = MemoryId::new(4);
pub static REGISTRY_MEMORY_ID: MemoryId = MemoryId::new(5);

thread_local! {
    pub static MEMORY_MANAGER: MemoryManagerStorage = init_memory_manager();
    pub static PROXIES: CellStorageRef<Principals> = init_cell(&MEMORY_MANAGER, "proxies", PROXIES_MEMORY_ID);
    pub static SHARDS: CellStorageRef<ShardsIndex> = init_cell(&MEMORY_MANAGER, "shards", SHARDS_MEMORY_ID);
    pub static SHARD_ITER: CellStorageRef<Principal> = init_cell(&MEMORY_MANAGER, "shard_iter", SHARD_ITER_MEMORY_ID);
    pub static KEY_ITER: CellStorageRef<Key> = init_cell(&MEMORY_MANAGER, "key_iter", KEY_ITER_MEMORY_ID);
    pub static SHARD_WASM: CellStorageRef<Vec<u8>> = init_cell(&MEMORY_MANAGER, "shards_wasm", SHARD_WASM_MEMORY_ID);
    pub static REGISTRY: StorageRef<Key, Principal> = init_btree(&MEMORY_MANAGER, REGISTRY_MEMORY_ID);
}
