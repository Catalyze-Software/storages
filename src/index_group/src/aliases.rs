use catalyze_shared::group_with_members::{GroupEntry, GroupFilter, GroupSort, GroupWithMembers};

#[allow(dead_code)]
pub const CANDID_PATH: &str = "index_group";

pub type Key = u64;
pub type Value = GroupWithMembers;
pub type Entry = GroupEntry;
pub type EntryFilter = GroupFilter;
pub type EntrySort = GroupSort;
