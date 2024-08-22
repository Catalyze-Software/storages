use catalyze_shared::{
    state::{init_btree, init_cell, init_memory_manager},
    CellStorageRef, MemoryManagerStorage, StorageRef,
};
use common::Principals;
use ic_stable_structures::memory_manager::MemoryId;

use crate::aliases::{Key, Value};

pub static PROXIES_MEMORY_ID: MemoryId = MemoryId::new(0);
pub static KEY_ITER_MEMORY_ID: MemoryId = MemoryId::new(1);
pub static DATA_MEMORY_ID: MemoryId = MemoryId::new(2);

thread_local! {
    pub static MEMORY_MANAGER: MemoryManagerStorage = init_memory_manager();
    pub static PROXIES: CellStorageRef<Principals> = init_cell(&MEMORY_MANAGER, "proxies", PROXIES_MEMORY_ID);
    pub static KEY_ITER: CellStorageRef<Key> = init_cell(&MEMORY_MANAGER, "key_iter", KEY_ITER_MEMORY_ID);
    pub static DATA: StorageRef<Key, Value> = init_btree(&MEMORY_MANAGER, DATA_MEMORY_ID);
}
