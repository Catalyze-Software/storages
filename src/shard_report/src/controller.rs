use candid::Principal;
use catalyze_shared::{StaticCellStorageRef, StaticStorageRef};
use common::ShardController;

use crate::{
    aliases::{EntryFilter, Key, Value, DATA_KIND},
    state::{DATA, INDEX},
};

pub struct Controller;

impl ShardController<Key, Value, EntryFilter> for Controller {
    fn name(&self) -> String {
        DATA_KIND.to_owned()
    }

    fn storage_index(&self) -> StaticCellStorageRef<Principal> {
        &INDEX
    }

    fn storage_raw(&self) -> StaticStorageRef<Key, Value> {
        &DATA
    }
}

pub fn controller() -> impl ShardController<Key, Value, EntryFilter> {
    Controller
}
