use candid::Principal;
use catalyze_shared::profile::{Profile, ProfileEntry, ProfileFilter, ProfileSort};

#[allow(dead_code)]
pub const CANDID_PATH: &str = "index_profile";

pub type Key = Principal;
pub type Value = Profile;
pub type Entry = ProfileEntry;
pub type EntryFilter = ProfileFilter;
pub type EntrySort = ProfileSort;
