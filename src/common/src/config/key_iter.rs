use catalyze_shared::{CellStorage, StaticCellStorageRef};

use crate::IDIter;

pub struct KeyIter {
    name: String,
    storage: StaticCellStorageRef<u64>,
}

impl KeyIter {
    pub fn new(storage: StaticCellStorageRef<u64>) -> Self {
        Self {
            name: "id_iter".to_owned(),
            storage,
        }
    }
}

impl CellStorage<u64> for KeyIter {
    fn name(&self) -> String {
        self.name.clone()
    }

    fn storage(&self) -> StaticCellStorageRef<u64> {
        self.storage
    }
}

impl IDIter for KeyIter {}
