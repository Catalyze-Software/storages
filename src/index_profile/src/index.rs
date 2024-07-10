use candid::Principal;
use catalyze_shared::profile::Profile;
use common::{CellStorage, IDMap, Principals, ShardClient, StorageIndex};

use crate::storage::{IDs, ShardIter, Shards};

#[derive(Default)]
pub struct ProfileShardClient;

impl ShardClient<Principal, Profile> for ProfileShardClient {}

#[derive(Default)]
pub struct ProfileIndex;

impl StorageIndex<Principal, Profile> for ProfileIndex {
    fn shards(&self) -> impl CellStorage<Principals> {
        Shards::default()
    }

    fn shard_iter(&self) -> impl CellStorage<Principal> {
        ShardIter::default()
    }

    fn ids(&self) -> impl IDMap<Principal> {
        IDs::default()
    }

    fn client(&self) -> impl ShardClient<Principal, Profile> {
        ProfileShardClient
    }
}
