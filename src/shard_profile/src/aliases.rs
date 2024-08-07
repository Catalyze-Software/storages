use candid::Principal;
use catalyze_shared::profile::{Profile, ProfileEntry, ProfileFilter};

pub const DATA_KIND: &str = "profile";

pub type Key = Principal;
pub type Value = Profile;
pub type Entry = ProfileEntry;
pub type EntryFilter = ProfileFilter;
