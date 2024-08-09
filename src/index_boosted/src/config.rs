use catalyze_shared::StaticCellStorageRef;
use common::{IndexConfigBase, IndexConfigWithKeyIter, Principals};

use crate::{aliases::Key, state::*};

#[derive(Clone)]
pub struct Config {
    proxies: StaticCellStorageRef<Principals>,
    key_iter: StaticCellStorageRef<Key>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            proxies: &PROXIES,
            key_iter: &KEY_ITER,
        }
    }
}

impl IndexConfigBase<Key> for Config {
    fn storage_proxies(&self) -> StaticCellStorageRef<Principals> {
        self.proxies
    }
}

impl IndexConfigWithKeyIter for Config {
    fn storage_key_iter(&self) -> StaticCellStorageRef<Key> {
        self.key_iter
    }
}

pub fn config() -> Config {
    Config::default()
}
