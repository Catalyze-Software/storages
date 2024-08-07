use crate::{CellStorage, ShardsIndex, StaticCellStorageRef};

pub struct Shards {
    name: String,
    storage: StaticCellStorageRef<ShardsIndex>,
}

impl Shards {
    pub fn new(storage: StaticCellStorageRef<ShardsIndex>) -> Self {
        Self {
            name: "shards".to_owned(),
            storage,
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
