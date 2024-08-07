use common::ShardController;

use crate::{
    aliases::{EntryFilter, Key, Value},
    storage::ProfileStorage,
};

pub struct ProfileController;

impl ShardController<Key, Value, EntryFilter> for ProfileController {
    fn storage(&self) -> impl common::ShardStorage<Key, Value> {
        ProfileStorage::default()
    }
}
