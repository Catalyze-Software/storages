use catalyze_shared::StaticStorageRef;
use common::IndexControllerStateful;

use crate::{
    aliases::{EntryFilter, EntrySort, Key, Value, DATA_KIND},
    state::DATA,
};

#[derive(Default)]
pub struct Controller;

impl IndexControllerStateful<Key, Value, EntryFilter, EntrySort> for Controller {
    fn name(&self) -> String {
        DATA_KIND.to_owned()
    }

    fn raw(&self) -> StaticStorageRef<Key, Value> {
        &DATA
    }
}

pub fn controller() -> Controller {
    Controller
}
