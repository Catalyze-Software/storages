use std::cell::RefCell;

use candid::Principal;
use catalyze_shared::{MemoryManagerStorage, StorageRef};
use common::CellStorageRef;
use ic_stable_structures::{
    memory_manager::{MemoryId, MemoryManager},
    Cell, DefaultMemoryImpl, StableBTreeMap,
};

use crate::aliases::{Key, Value};

pub static INDEX_MEMORY_ID: MemoryId = MemoryId::new(0);
pub static PROFILES_MEMORY_ID: MemoryId = MemoryId::new(1);

thread_local! {
    pub static MEMORY_MANAGER: MemoryManagerStorage =
        RefCell::new(MemoryManager::init(DefaultMemoryImpl::default()));

    pub static INDEX: CellStorageRef<Principal> = RefCell::new(
        Cell::init(MEMORY_MANAGER.with(|p| p.borrow().get(INDEX_MEMORY_ID)), None).expect("Failed to initialize index cell")
    );

    pub static PROFILES: StorageRef<Key, Value> = RefCell::new(
        StableBTreeMap::init(MEMORY_MANAGER.with(|p| p.borrow().get(PROFILES_MEMORY_ID)))
    );
}
