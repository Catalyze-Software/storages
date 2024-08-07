use common::{CellStorage, ShardsIndex, StaticCellStorageRef};

use super::state::SHARDS;

pub struct Shards {
    name: String,
    storage: StaticCellStorageRef<ShardsIndex>,
}

impl Default for Shards {
    fn default() -> Self {
        Self {
            name: "shards".to_owned(),
            storage: &SHARDS,
        }
    }
}
impl CellStorage<ShardsIndex> for Shards {
    fn name(&self) -> String {
        self.name.clone()
    }

    fn storage(&self) -> StaticCellStorageRef<ShardsIndex> {
        self.storage
    }
}
