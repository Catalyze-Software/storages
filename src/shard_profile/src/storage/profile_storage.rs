use catalyze_shared::StaticStorageRef;
use common::ShardStorage;

use crate::aliases::{Key, Value};

use super::PROFILES;

pub struct ProfileStorage {
    pub name: String,
    pub raw: StaticStorageRef<Key, Value>,
}

impl Default for ProfileStorage {
    fn default() -> Self {
        Self {
            name: "profiles".to_owned(),
            raw: &PROFILES,
        }
    }
}

impl ShardStorage<Key, Value> for ProfileStorage {
    fn name(&self) -> String {
        self.name.clone()
    }

    fn storage(&self) -> StaticStorageRef<Key, Value> {
        self.raw
    }
}
