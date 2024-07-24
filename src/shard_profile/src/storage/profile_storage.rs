use candid::Principal;
use catalyze_shared::{profile::Profile, StaticStorageRef};
use common::ShardStorage;

use super::PROFILES;

pub struct ProfileStorage {
    pub name: String,
    pub raw: StaticStorageRef<Principal, Profile>,
}

impl Default for ProfileStorage {
    fn default() -> Self {
        Self {
            name: "profiles".to_owned(),
            raw: &PROFILES,
        }
    }
}

impl ShardStorage<Principal, Profile> for ProfileStorage {
    fn name(&self) -> String {
        self.name.clone()
    }

    fn storage(&self) -> StaticStorageRef<Principal, Profile> {
        self.raw
    }
}
