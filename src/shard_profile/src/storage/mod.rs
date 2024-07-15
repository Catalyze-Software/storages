mod index;
mod profile_storage;
mod state;

use catalyze_shared::{profile::Profile, StaticStorageRef};
use common::ShardStorage;
pub use index::*;
pub use profile_storage::*;
pub use state::*;

use candid::Principal;

pub struct ProfileStore {
    pub name: String,
    pub raw: StaticStorageRef<Principal, Profile>,
}

impl Default for ProfileStore {
    fn default() -> Self {
        Self {
            name: "profiles".to_owned(),
            raw: &PROFILES,
        }
    }
}

impl ShardStorage<Principal, Profile> for ProfileStore {
    fn name(&self) -> String {
        self.name.clone()
    }

    fn storage(&self) -> StaticStorageRef<Principal, Profile> {
        self.raw
    }
}
