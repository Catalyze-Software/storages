use candid::Principal;
use catalyze_shared::StaticStorageRef;
use common::IDMap;

use super::state::IDS;

pub struct IDs {
    name: String,
    storage: StaticStorageRef<Principal, Principal>,
}

impl Default for IDs {
    fn default() -> Self {
        Self {
            name: "ids".to_owned(),
            storage: &IDS,
        }
    }
}

impl IDMap<Principal> for IDs {
    fn name(&self) -> String {
        self.name.clone()
    }

    fn raw(&self) -> StaticStorageRef<Principal, Principal> {
        self.storage
    }
}
