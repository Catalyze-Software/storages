use common::{CellStorage, Principals, StaticCellStorageRef};

use super::state::SHARDS;

pub struct Shards {
    name: String,
    storage: StaticCellStorageRef<Principals>,
}

impl Default for Shards {
    fn default() -> Self {
        Self {
            name: "shards".to_owned(),
            storage: &SHARDS,
        }
    }
}
impl CellStorage<Principals> for Shards {
    fn name(&self) -> String {
        self.name.clone()
    }

    fn storage(&self) -> StaticCellStorageRef<Principals> {
        self.storage
    }
}
