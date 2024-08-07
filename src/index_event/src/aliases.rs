use catalyze_shared::event::{Event, EventEntry, EventFilter, EventSort};

#[allow(dead_code)]
pub const DATA_KIND: &str = "event";

pub type Key = u64;
pub type Value = Event;
pub type Entry = EventEntry;
pub type EntryFilter = EventFilter;
pub type EntrySort = EventSort;
