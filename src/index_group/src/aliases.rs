use catalyze_shared::{
    group::{Group, GroupEntry},
    group_with_members::{GroupFilter, GroupSort},
};

#[allow(dead_code)]
pub const CANDID_PATH: &str = "index_group";

pub type Key = u64;
pub type Value = Group;
pub type Entry = GroupEntry;
pub type EntryFilter = GroupFilter;
pub type EntrySort = GroupSort;
