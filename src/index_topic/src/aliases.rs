use catalyze_shared::topic::{Topic, TopicEntry, TopicFilter, TopicSort};

#[allow(dead_code)]
pub const DATA_KIND: &str = "topic";

pub type Key = u64;
pub type Value = Topic;
pub type Entry = TopicEntry;
pub type EntryFilter = TopicFilter;
pub type EntrySort = TopicSort;
