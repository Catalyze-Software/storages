use std::cell::RefCell;

use candid::Principal;
use catalyze_shared::{MemoryManagerStorage, StorageRef};
use common::{CellStorageRef, Principals, ShardsIndex};
use ic_stable_structures::{
    memory_manager::{MemoryId, MemoryManager},
    Cell, DefaultMemoryImpl, StableBTreeMap,
};

pub static PROXIES_MEMORY_ID: MemoryId = MemoryId::new(0);
pub static SHARDS_MEMORY_ID: MemoryId = MemoryId::new(1);
pub static SHARD_ITER_MEMORY_ID: MemoryId = MemoryId::new(2);
pub static ID_ITER_MEMORY_ID: MemoryId = MemoryId::new(3);
pub static SHARD_WASM_MEMORY_ID: MemoryId = MemoryId::new(4);

pub static IDS_MEMORY_ID: MemoryId = MemoryId::new(5);

thread_local! {
    pub static MEMORY_MANAGER: MemoryManagerStorage =
        RefCell::new(MemoryManager::init(DefaultMemoryImpl::default()));

    pub static PROXIES: CellStorageRef<Principals> = RefCell::new(
        Cell::init(MEMORY_MANAGER.with(|p| p.borrow().get(PROXIES_MEMORY_ID)), None).expect("Failed to initialize proxies cell")
    );

    pub static SHARDS: CellStorageRef<ShardsIndex> = RefCell::new(
        Cell::init(MEMORY_MANAGER.with(|p| p.borrow().get(SHARDS_MEMORY_ID)), None).expect("Failed to initialize shards cell")
    );

    pub static SHARD_ITER: CellStorageRef<Principal> = RefCell::new(
        Cell::init(MEMORY_MANAGER.with(|p| p.borrow().get(SHARD_ITER_MEMORY_ID)), None).expect("Failed to initialize shard iter cell")
    );

    pub static ID_ITER: CellStorageRef<u64> = RefCell::new(
        Cell::init(MEMORY_MANAGER.with(|p| p.borrow().get(ID_ITER_MEMORY_ID)), None).expect("Failed to initialize id iter cell")
    );

    pub static SHARD_WASM: CellStorageRef<Vec<u8>> = RefCell::new(
        Cell::init(MEMORY_MANAGER.with(|p| p.borrow().get(SHARD_WASM_MEMORY_ID)), None).expect("Failed to initialize shards wasm cell")
    );

    pub static IDS: StorageRef<u64, Principal> = RefCell::new(
        StableBTreeMap::init(MEMORY_MANAGER.with(|p| p.borrow().get(IDS_MEMORY_ID)))
    );
}
