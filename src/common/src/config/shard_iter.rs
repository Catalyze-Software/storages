use crate::{CellStorage, StaticCellStorageRef};
use candid::Principal;

pub struct ShardIter {
    name: String,
    storage: StaticCellStorageRef<Principal>,
}

impl ShardIter {
    pub fn new(storage: StaticCellStorageRef<Principal>) -> Self {
        Self {
            name: "shard_iter".to_owned(),
            storage,
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
