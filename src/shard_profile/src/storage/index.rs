use candid::Principal;
use common::{CellStorage, StaticCellStorageRef};

use super::INDEX;

pub struct Index {
    name: String,
    storage: StaticCellStorageRef<Principal>,
}

impl Default for Index {
    fn default() -> Self {
        Self {
            name: "index".to_owned(),
            storage: &INDEX,
        }
    }
}

impl CellStorage<Principal> for Index {
    fn name(&self) -> String {
        self.name.clone()
    }

    fn storage(&self) -> StaticCellStorageRef<Principal> {
        self.storage
    }
}
