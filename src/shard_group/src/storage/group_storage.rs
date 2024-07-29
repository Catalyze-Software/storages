use catalyze_shared::{group::Group, StaticStorageRef};
use common::ShardStorage;

use super::GROUPS;

pub struct GroupStorage {
    pub name: String,
    pub raw: StaticStorageRef<u64, Group>,
}

impl Default for GroupStorage {
    fn default() -> Self {
        Self {
            name: "group".to_owned(),
            raw: &GROUPS,
        }
    }
}

impl ShardStorage<u64, Group> for GroupStorage {
    fn name(&self) -> String {
        self.name.clone()
    }

    fn storage(&self) -> StaticStorageRef<u64, Group> {
        self.raw
    }
}
