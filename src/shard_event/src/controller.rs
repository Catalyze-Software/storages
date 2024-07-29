use catalyze_shared::event::{Event, EventFilter};
use common::ShardController;

use crate::storage::EventStorage;

pub struct EventController;

impl ShardController<u64, Event, EventFilter> for EventController {
    fn storage(&self) -> impl common::ShardStorage<u64, Event> {
        EventStorage::default()
    }
}
