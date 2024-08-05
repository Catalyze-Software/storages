use candid::Principal;
use common::{CellStorage, StaticCellStorageRef};

use super::state::SHARD_ITER;

pub struct ShardIter {
    name: String,
    storage: StaticCellStorageRef<Principal>,
}

impl Default for ShardIter {
    fn default() -> Self {
        Self {
            name: "shard_iter".to_owned(),
            storage: &SHARD_ITER,
        }
    }
}

impl CellStorage<Principal> for ShardIter {
    fn name(&self) -> String {
        self.name.clone()
    }

    fn storage(&self) -> StaticCellStorageRef<Principal> {
        self.storage
    }
}
