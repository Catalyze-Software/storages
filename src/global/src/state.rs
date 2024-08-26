use catalyze_shared::{
    reward::RewardableActivity,
    state::{init_btree, init_cell, init_memory_manager},
    CellStorage, CellStorageRef, GenericCellStorage, MemoryManagerStorage, StorageRef,
};
use common::{Principals, ShardStorage, Storage};
use ic_stable_structures::{memory_manager::MemoryId, StableBTreeMap};

pub static PROXIES_MEMORY_ID: MemoryId = MemoryId::new(0);
pub static HISTORY_POINT_MEMORY_ID: MemoryId = MemoryId::new(1);
pub static ID_STORAGE_MEMORY_ID: MemoryId = MemoryId::new(2);
pub static REWARD_BUFFER_MEMORY_ID: MemoryId = MemoryId::new(3);

thread_local! {
    pub static MEMORY_MANAGER: MemoryManagerStorage = init_memory_manager();
    pub static PROXIES: CellStorageRef<Principals> = init_cell(&MEMORY_MANAGER, "proxies", PROXIES_MEMORY_ID);
    pub static ID_STORAGE: StorageRef<String, u64> = init_btree(&MEMORY_MANAGER, ID_STORAGE_MEMORY_ID);
    pub static REWARD_BUFFER: StorageRef<u64, RewardableActivity> = init_btree(&MEMORY_MANAGER, REWARD_BUFFER_MEMORY_ID);
    pub static HISTORY_POINT: CellStorageRef<u64> = init_cell(&MEMORY_MANAGER, "history_point", HISTORY_POINT_MEMORY_ID);
}

pub fn history_point() -> impl CellStorage<u64> {
    GenericCellStorage::new("history_point", &HISTORY_POINT)
}

pub fn rewards_buffer() -> impl ShardStorage<u64, RewardableActivity> {
    Storage::new("rewards_buffer", &REWARD_BUFFER)
}

pub fn clear_reward_buffer() {
    REWARD_BUFFER.with(|n| {
        n.replace(StableBTreeMap::new(
            MEMORY_MANAGER.with(|m| m.borrow().get(REWARD_BUFFER_MEMORY_ID)),
        ))
    });
}

pub fn proxies() -> impl CellStorage<Principals> {
    GenericCellStorage::new("proxies", &PROXIES)
}

pub fn id_storage() -> impl ShardStorage<String, u64> {
    Storage::new("ids", &ID_STORAGE)
}
