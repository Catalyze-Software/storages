use candid::Principal;
use catalyze_shared::{
    event::{Event, EventFilter, EventSort},
    Sorter,
};
use common::{CellStorage, IDMap, IndexController, ShardClient, ShardsIndex};

use crate::storage::{IDs, ShardIter, Shards};

#[derive(Default)]
pub struct EventShardClient;

impl ShardClient<u64, Event, EventFilter> for EventShardClient {}

#[derive(Default)]
pub struct EventIndex;

impl IndexController<u64, Event, EventFilter> for EventIndex {
    fn shards(&self) -> impl CellStorage<ShardsIndex> {
        Shards::default()
    }

    fn shard_iter(&self) -> impl CellStorage<Principal> {
        ShardIter::default()
    }

    fn ids(&self) -> impl IDMap<u64> {
        IDs::default()
    }

    fn client(&self) -> impl ShardClient<u64, Event, EventFilter> {
        EventShardClient
    }

    fn sorter(&self) -> impl Sorter<u64, Event> {
        EventSort::default()
    }
}
