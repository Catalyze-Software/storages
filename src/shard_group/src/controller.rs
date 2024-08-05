use catalyze_shared::group::{Group, GroupFilter};
use common::ShardController;

use crate::storage::GroupStorage;

pub struct GroupController;

impl ShardController<u64, Group, GroupFilter> for GroupController {
    fn storage(&self) -> impl common::ShardStorage<u64, Group> {
        GroupStorage::default()
    }
}
