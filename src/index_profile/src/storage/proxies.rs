use common::{CellStorage, Principals, StaticCellStorageRef};

use super::state::PROXIES;

pub struct Proxies {
    name: String,
    storage: StaticCellStorageRef<Principals>,
}

impl Default for Proxies {
    fn default() -> Self {
        Self {
            name: "proxies".to_owned(),
            storage: &PROXIES,
        }
    }
}

impl CellStorage<Principals> for Proxies {
    fn name(&self) -> String {
        self.name.clone()
    }

    fn storage(&self) -> StaticCellStorageRef<Principals> {
        self.storage
    }
}
