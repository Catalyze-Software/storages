use candid::Principal;
use catalyze_shared::{
    group::{Group, GroupFilter, GroupSort},
    Sorter,
};
use common::{CellStorage, IDMap, IndexController, ShardClient, ShardsIndex};

use crate::storage::{IDs, ShardIter, Shards};

#[derive(Default)]
pub struct GroupShardClient;

impl ShardClient<u64, Group, GroupFilter> for GroupShardClient {}

#[derive(Default)]
pub struct GroupIndex;

impl IndexController<u64, Group, GroupFilter> for GroupIndex {
    fn shards(&self) -> impl CellStorage<ShardsIndex> {
        Shards::default()
    }

    fn shard_iter(&self) -> impl CellStorage<Principal> {
        ShardIter::default()
    }

    fn ids(&self) -> impl IDMap<u64> {
        IDs::default()
    }

    fn client(&self) -> impl ShardClient<u64, Group, GroupFilter> {
        GroupShardClient
    }

    fn sorter(&self) -> impl Sorter<u64, Group> {
        GroupSort::default()
    }
}
