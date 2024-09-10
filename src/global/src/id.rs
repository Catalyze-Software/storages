use catalyze_shared::{CanisterResult, StorageRef};
use common::ShardStorage;
use ic_stable_structures::Storable;

use crate::state::{self};

#[derive(Debug, Clone)]
pub enum IDKind {
    RewardBuffer,
}

impl std::fmt::Display for IDKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            IDKind::RewardBuffer => write!(f, "rewards_buffer"),
        }
    }
}

pub fn next(k: IDKind) -> CanisterResult<u64> {
    // Use the old id if the id store is empty (only needed for existing data)
    let id = state::id_storage()
        .get_opt(k.to_string())
        .map(|(_, v)| v)
        .unwrap_or_else(|| get_last(k.clone()))
        + 1;

    state::id_storage()
        .insert_by_key(k.to_string(), id)
        .map(|(_, v)| v)
}

fn get_last(kind: IDKind) -> u64 {
    match kind {
        IDKind::RewardBuffer => state::rewards_buffer().storage().with(last_key),
    }
}

fn last_key<T: Storable>(data: &StorageRef<u64, T>) -> u64 {
    data.borrow().last_key_value().map(|(k, _)| k).unwrap_or(1)
}
