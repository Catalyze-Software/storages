use std::cell::RefCell;

use candid::Principal;
use catalyze_shared::{group::Group, MemoryManagerStorage, StorageRef};
use common::CellStorageRef;
use ic_stable_structures::{
    memory_manager::{MemoryId, MemoryManager},
    Cell, DefaultMemoryImpl, StableBTreeMap,
};

pub static INDEX_MEMORY_ID: MemoryId = MemoryId::new(0);
pub static GROUP_MEMORY_ID: MemoryId = MemoryId::new(1);

thread_local! {
    pub static MEMORY_MANAGER: MemoryManagerStorage =
        RefCell::new(MemoryManager::init(DefaultMemoryImpl::default()));

    pub static INDEX: CellStorageRef<Principal> = RefCell::new(
        Cell::init(MEMORY_MANAGER.with(|p| p.borrow().get(INDEX_MEMORY_ID)), None).expect("Failed to initialize index cell")
    );

    pub static GROUPS: StorageRef<u64, Group> = RefCell::new(
        StableBTreeMap::init(MEMORY_MANAGER.with(|p| p.borrow().get(GROUP_MEMORY_ID)))
    );
}
