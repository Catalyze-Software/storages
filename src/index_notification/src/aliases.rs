use catalyze_shared::notification::{
    Notification, NotificationEntry, NotificationFilter, NotificationSort,
};

#[allow(dead_code)]
pub const DATA_KIND: &str = "notification";

pub type Key = u64;
pub type Value = Notification;
pub type Entry = NotificationEntry;
pub type EntryFilter = NotificationFilter;
pub type EntrySort = NotificationSort;
