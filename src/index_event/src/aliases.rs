use catalyze_shared::event_with_attendees::{
    EventEntry, EventFilter, EventSort, EventWithAttendees,
};

#[allow(dead_code)]
pub const DATA_KIND: &str = "event";

pub type Key = u64;
pub type Value = EventWithAttendees;
pub type Entry = EventEntry;
pub type EntryFilter = EventFilter;
pub type EntrySort = EventSort;
