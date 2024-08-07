use catalyze_shared::group::{Group, GroupEntry, GroupFilter};

pub const DATA_KIND: &str = "group";

pub type Key = u64;
pub type Value = Group;
pub type Entry = GroupEntry;
pub type EntryFilter = GroupFilter;
