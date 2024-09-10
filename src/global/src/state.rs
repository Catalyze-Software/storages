use candid::Principal;
use catalyze_shared::{
    reward::RewardableActivity,
    state::{init_btree, init_cell, init_memory_manager},
    CellStorage, CellStorageRef, GenericCellStorage, MemoryManagerStorage, StorageRef,
};
use common::{Principals, ShardStorage, Storage};
use ic_stable_structures::memory_manager::MemoryId;

pub static PROXIES_MEMORY_ID: MemoryId = MemoryId::new(0);
pub static HISTORY_POINT_MEMORY_ID: MemoryId = MemoryId::new(1);
pub static ID_STORAGE_MEMORY_ID: MemoryId = MemoryId::new(2);
pub static REWARD_BUFFER_MEMORY_ID: MemoryId = MemoryId::new(3);
pub static REWARD_TIMER_MEMORY_ID: MemoryId = MemoryId::new(4);

pub static REWARD_CANISTER_MEMORY_ID: MemoryId = MemoryId::new(5);
pub static GROUP_CANISTER_MEMORY_ID: MemoryId = MemoryId::new(6);

thread_local! {
    pub static MEMORY_MANAGER: MemoryManagerStorage = init_memory_manager();
    pub static PROXIES: CellStorageRef<Principals> = init_cell(&MEMORY_MANAGER, "proxies", PROXIES_MEMORY_ID);
    pub static ID_STORAGE: StorageRef<String, u64> = init_btree(&MEMORY_MANAGER, ID_STORAGE_MEMORY_ID);
    pub static REWARD_BUFFER: StorageRef<u64, RewardableActivity> = init_btree(&MEMORY_MANAGER, REWARD_BUFFER_MEMORY_ID);
    pub static HISTORY_POINT: CellStorageRef<u64> = init_cell(&MEMORY_MANAGER, "history_point", HISTORY_POINT_MEMORY_ID);

    pub static REWARD_TIMER: CellStorageRef<u64> = init_cell(&MEMORY_MANAGER, "reward_timer", REWARD_TIMER_MEMORY_ID);

    pub static REWARD_CANISTER: CellStorageRef<Principal> = init_cell(&MEMORY_MANAGER, "reward_canister_id", REWARD_CANISTER_MEMORY_ID);
    pub static GROUP_CANISTER: CellStorageRef<Principal> = init_cell(&MEMORY_MANAGER, "group_canister_id", GROUP_CANISTER_MEMORY_ID);
}

pub fn history_point() -> impl CellStorage<u64> {
    GenericCellStorage::new("history_point", &HISTORY_POINT)
}

pub fn rewards_buffer() -> impl ShardStorage<u64, RewardableActivity> {
    Storage::new("rewards_buffer", &REWARD_BUFFER)
}

pub fn proxies() -> impl CellStorage<Principals> {
    GenericCellStorage::new("proxies", &PROXIES)
}

pub fn id_storage() -> impl ShardStorage<String, u64> {
    Storage::new("ids", &ID_STORAGE)
}

pub fn reward_timer() -> impl CellStorage<u64> {
    GenericCellStorage::new("reward_timer", &REWARD_TIMER)
}

pub fn reward_canister() -> impl CellStorage<Principal> {
    GenericCellStorage::new("reward_canister_id", &REWARD_CANISTER)
}

pub fn group_canister() -> impl CellStorage<Principal> {
    GenericCellStorage::new("group_canister_id", &GROUP_CANISTER)
}
