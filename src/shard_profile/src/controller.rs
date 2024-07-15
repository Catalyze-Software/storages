use candid::Principal;
use catalyze_shared::profile::{Profile, ProfileFilter};
use common::ShardController;

use crate::storage::ProfileStorage;

pub struct ProfileController;

impl ShardController<Principal, Profile, ProfileFilter> for ProfileController {
    fn storage(&self) -> impl common::ShardStorage<Principal, Profile> {
        ProfileStorage::default()
    }
}
