use common::{CellStorage, StaticCellStorageRef};

use crate::aliases::Key;

use super::INDEX;

pub struct Index {
    name: String,
    storage: StaticCellStorageRef<Key>,
}

impl Default for Index {
    fn default() -> Self {
        Self {
            name: "index".to_owned(),
            storage: &INDEX,
        }
    }
}

impl CellStorage<Key> for Index {
    fn name(&self) -> String {
        self.name.clone()
    }

    fn storage(&self) -> StaticCellStorageRef<Key> {
        self.storage
    }
}
