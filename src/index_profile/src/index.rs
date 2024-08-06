use candid::Principal;
use catalyze_shared::profile::{Profile, ProfileFilter, ProfileSort};
use common::{CellStorage, IDMap, IndexController, ShardClient, ShardsIndex};

use crate::storage::{IDs, ShardIter, Shards};

#[derive(Default)]
pub struct ProfileShardClient;

impl ShardClient<Principal, Profile, ProfileFilter> for ProfileShardClient {}

#[derive(Default)]
pub struct ProfileIndex;

impl IndexController<Principal, Profile, ProfileFilter, ProfileSort> for ProfileIndex {
    fn shards(&self) -> impl CellStorage<ShardsIndex> {
        Shards::default()
    }

    fn shard_iter(&self) -> impl CellStorage<Principal> {
        ShardIter::default()
    }

    fn ids(&self) -> impl IDMap<Principal> {
        IDs::default()
    }

    fn client(&self) -> impl ShardClient<Principal, Profile, ProfileFilter> {
        ProfileShardClient
    }
}
