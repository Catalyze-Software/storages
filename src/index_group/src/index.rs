use common::{IndexConfig, IndexController};

use crate::{
    aliases::{EntryFilter, EntrySort, Key, Value},
    config::Config,
};

#[derive(Default)]
pub struct Index {
    config: Config,
}

impl IndexController<Key, Value, EntryFilter, EntrySort> for Index {
    fn config(&self) -> impl IndexConfig<Key> {
        self.config.clone()
    }
}

pub fn index() -> Index {
    Index::default()
}
