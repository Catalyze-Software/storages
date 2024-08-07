use catalyze_shared::StaticStorageRef;
use common::ShardStorage;

use crate::aliases::{Key, Value};

use super::GROUPS;

pub struct GroupStorage {
    pub name: String,
    pub raw: StaticStorageRef<Key, Value>,
}

impl Default for GroupStorage {
    fn default() -> Self {
        Self {
            name: "group".to_owned(),
            raw: &GROUPS,
        }
    }
}

impl ShardStorage<Key, Value> for GroupStorage {
    fn name(&self) -> String {
        self.name.clone()
    }

    fn storage(&self) -> StaticStorageRef<Key, Value> {
        self.raw
    }
}
