use candid::Principal;
use catalyze_shared::profile_with_refs::{
    ProfileEntry, ProfileFilter, ProfileSort, ProfileWithRefs,
};

#[allow(dead_code)]
pub const CANDID_PATH: &str = "index_profile";

pub type Key = Principal;
pub type Value = ProfileWithRefs;
pub type Entry = ProfileEntry;
pub type EntryFilter = ProfileFilter;
pub type EntrySort = ProfileSort;
