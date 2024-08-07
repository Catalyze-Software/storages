use std::cell::RefCell;

use catalyze_shared::{MemoryManagerStorage, StorageRef};
use common::{CellStorageRef, Principals};
use ic_stable_structures::{
    memory_manager::{MemoryId, MemoryManager},
    Cell, DefaultMemoryImpl, StableBTreeMap,
};

use crate::aliases::{Key, Value};

pub static PROXIES_MEMORY_ID: MemoryId = MemoryId::new(0);
pub static KEY_ITER_MEMORY_ID: MemoryId = MemoryId::new(1);
pub static DATA_MEMORY_ID: MemoryId = MemoryId::new(2);

thread_local! {
    pub static MEMORY_MANAGER: MemoryManagerStorage =
        RefCell::new(MemoryManager::init(DefaultMemoryImpl::default()));

    pub static PROXIES: CellStorageRef<Principals> = RefCell::new(
        Cell::init(MEMORY_MANAGER.with(|p| p.borrow().get(PROXIES_MEMORY_ID)), None).expect("Failed to initialize proxies cell")
    );

    pub static KEY_ITER: CellStorageRef<Key> = RefCell::new(
        Cell::init(MEMORY_MANAGER.with(|p| p.borrow().get(KEY_ITER_MEMORY_ID)), None).expect("Failed to initialize key iter cell")
    );

    pub static DATA: StorageRef<Key, Value> = RefCell::new(
        StableBTreeMap::init(MEMORY_MANAGER.with(|p| p.borrow().get(DATA_MEMORY_ID)))
    );
}
