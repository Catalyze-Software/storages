use common::{IndexConfig, IndexController};

use crate::{
    aliases::{EntryFilter, EntrySort, Key, Value},
    config::Config,
};

#[derive(Default)]
pub struct Controller {
    config: Config,
}

impl IndexController<Key, Value, EntryFilter, EntrySort> for Controller {
    fn config(&self) -> impl IndexConfig<Key> {
        self.config.clone()
    }
}

pub fn controller() -> Controller {
    Controller::default()
}
