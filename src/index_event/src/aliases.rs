use catalyze_shared::event::{Event, EventEntry, EventFilter, EventSort};

#[allow(dead_code)]
pub const CANDID_PATH: &str = "index_event";

pub type Key = u64;
pub type Value = Event;
pub type Entry = EventEntry;
pub type EntryFilter = EventFilter;
pub type EntrySort = EventSort;
