use crate::{CellStorage, StaticCellStorageRef};

pub struct ShardWasm {
    name: String,
    storage: StaticCellStorageRef<Vec<u8>>,
}

impl ShardWasm {
    pub fn new(storage: StaticCellStorageRef<Vec<u8>>) -> Self {
        Self {
            name: "shard_wasm".to_owned(),
            storage,
        }
    }
}

impl CellStorage<Vec<u8>> for ShardWasm {
    fn name(&self) -> String {
        self.name.clone()
    }

    fn storage(&self) -> StaticCellStorageRef<Vec<u8>> {
        self.storage
    }
}
