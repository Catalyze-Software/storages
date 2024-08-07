use catalyze_shared::event::{Event, EventEntry, EventFilter};

pub const DATA_KIND: &str = "event";

pub type Key = u64;
pub type Value = Event;
pub type Entry = EventEntry;
pub type EntryFilter = EventFilter;
