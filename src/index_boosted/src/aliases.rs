use catalyze_shared::boosted::{Boost, BoostedEntry, BoostedFilter, BoostedSort};

#[allow(dead_code)]
pub const DATA_KIND: &str = "index_boosted";

pub type Key = u64;
pub type Value = Boost;
pub type Entry = BoostedEntry;
pub type EntryFilter = BoostedFilter;
pub type EntrySort = BoostedSort;
