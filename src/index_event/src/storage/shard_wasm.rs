use common::{CellStorage, StaticCellStorageRef};

use super::state::SHARD_WASM;

pub struct ShardWasm {
    name: String,
    storage: StaticCellStorageRef<Vec<u8>>,
}

impl Default for ShardWasm {
    fn default() -> Self {
        Self {
            name: "shard_wasm".to_owned(),
            storage: &SHARD_WASM,
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
