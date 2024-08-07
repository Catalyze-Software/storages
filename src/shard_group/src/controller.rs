use common::ShardController;

use crate::{
    aliases::{EntryFilter, Key, Value},
    storage::GroupStorage,
};

pub struct GroupController;

impl ShardController<Key, Value, EntryFilter> for GroupController {
    fn storage(&self) -> impl common::ShardStorage<Key, Value> {
        GroupStorage::default()
    }
}
