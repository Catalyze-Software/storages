use common::{CellStorage, IDIter, StaticCellStorageRef};

use super::state::ID_ITER;

pub struct IDIterator {
    name: String,
    storage: StaticCellStorageRef<u64>,
}

impl Default for IDIterator {
    fn default() -> Self {
        Self {
            name: "id_iter".to_owned(),
            storage: &ID_ITER,
        }
    }
}

impl CellStorage<u64> for IDIterator {
    fn name(&self) -> String {
        self.name.clone()
    }

    fn storage(&self) -> StaticCellStorageRef<u64> {
        self.storage
    }
}

impl IDIter for IDIterator {}
