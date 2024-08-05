use candid::Principal;
use catalyze_shared::{
    report::{Report, ReportFilter, ReportSort},
    Sorter,
};
use common::{CellStorage, IDMap, IndexController, ShardClient, ShardsIndex};

use crate::storage::{IDs, ShardIter, Shards};

#[derive(Default)]
pub struct ReportShardClient;

impl ShardClient<u64, Report, ReportFilter> for ReportShardClient {}

#[derive(Default)]
pub struct ReportIndex;

impl IndexController<u64, Report, ReportFilter> for ReportIndex {
    fn shards(&self) -> impl CellStorage<ShardsIndex> {
        Shards::default()
    }

    fn shard_iter(&self) -> impl CellStorage<Principal> {
        ShardIter::default()
    }

    fn ids(&self) -> impl IDMap<u64> {
        IDs::default()
    }

    fn client(&self) -> impl ShardClient<u64, Report, ReportFilter> {
        ReportShardClient
    }

    fn sorter(&self) -> impl Sorter<u64, Report> {
        ReportSort::default()
    }
}
