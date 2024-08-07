use catalyze_shared::StaticStorageRef;
use common::IndexControllerStateful;

use crate::{
    aliases::{EntryFilter, EntrySort, Key, Value, CANDID_PATH},
    state::DATA,
};

#[derive(Default)]
pub struct Controller;

impl IndexControllerStateful<Key, Value, EntryFilter, EntrySort> for Controller {
    fn name(&self) -> String {
        CANDID_PATH.to_owned()
    }

    fn raw(&self) -> StaticStorageRef<Key, Value> {
        &DATA
    }
}

pub fn controller() -> Controller {
    Controller::default()
}
