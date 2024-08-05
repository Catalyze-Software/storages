use candid::Principal;
use catalyze_shared::StaticStorageRef;
use common::IDMap;

use super::state::IDS;

pub struct IDs {
    name: String,
    storage: StaticStorageRef<u64, Principal>,
}

impl Default for IDs {
    fn default() -> Self {
        Self {
            name: "ids".to_owned(),
            storage: &IDS,
        }
    }
}

impl IDMap<u64> for IDs {
    fn name(&self) -> String {
        self.name.clone()
    }

    fn storage(&self) -> StaticStorageRef<u64, Principal> {
        self.storage
    }
}
