use candid::Principal;
use catalyze_shared::profile_with_refs::{
    ProfileEntry, ProfileFilter, ProfileSort, ProfileWithRefs,
};

pub const DATA_KIND: &str = "profile";

pub type Key = Principal;
pub type Value = ProfileWithRefs;
pub type Entry = ProfileEntry;
pub type EntryFilter = ProfileFilter;
#[allow(dead_code)]
pub type EntrySort = ProfileSort;
