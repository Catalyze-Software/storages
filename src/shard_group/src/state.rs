use candid::Principal;
use catalyze_shared::{
    state::{init_btree, init_cell, init_memory_manager},
    CellStorageRef, MemoryManagerStorage, StorageRef,
};
use ic_stable_structures::memory_manager::MemoryId;

use crate::aliases::{Key, Value};

pub static INDEX_MEMORY_ID: MemoryId = MemoryId::new(0);
pub static DATA_MEMORY_ID: MemoryId = MemoryId::new(1);

thread_local! {
    pub static MEMORY_MANAGER: MemoryManagerStorage = init_memory_manager();
    pub static INDEX: CellStorageRef<Principal> = init_cell(&MEMORY_MANAGER, "index", INDEX_MEMORY_ID);
    pub static DATA: StorageRef<Key, Value> = init_btree(&MEMORY_MANAGER, DATA_MEMORY_ID);
}
