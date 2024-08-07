use crate::{CellStorage, Principals, StaticCellStorageRef};

pub struct Proxies {
    name: String,
    storage: StaticCellStorageRef<Principals>,
}

impl Proxies {
    pub fn new(storage: StaticCellStorageRef<Principals>) -> Self {
        Self {
            name: "proxies".to_owned(),
            storage,
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
