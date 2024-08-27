use catalyze_shared::friend_request::{
    FriendRequest, FriendRequestEntry, FriendRequestFilter, FriendRequestSort,
};

#[allow(dead_code)]
pub const DATA_KIND: &str = "friend_request";

pub type Key = u64;
pub type Value = FriendRequest;
pub type Entry = FriendRequestEntry;
pub type EntryFilter = FriendRequestFilter;
pub type EntrySort = FriendRequestSort;
