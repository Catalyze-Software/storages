use catalyze_shared::{event::Event, StaticStorageRef};
use common::ShardStorage;

use super::EVENTS;

pub struct EventStorage {
    pub name: String,
    pub raw: StaticStorageRef<u64, Event>,
}

impl Default for EventStorage {
    fn default() -> Self {
        Self {
            name: "event".to_owned(),
            raw: &EVENTS,
        }
    }
}

impl ShardStorage<u64, Event> for EventStorage {
    fn name(&self) -> String {
        self.name.clone()
    }

    fn storage(&self) -> StaticStorageRef<u64, Event> {
        self.raw
    }
}
