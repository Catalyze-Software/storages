use candid::Principal;
use common::{CellStorage, IDMap, Principals};

use super::{IDs, Proxies, ShardIter, Shards};

pub struct Storages;

impl Storages {
    pub fn proxies() -> impl CellStorage<Principals> {
        Proxies::default()
    }

    pub fn shards() -> impl CellStorage<Principals> {
        Shards::default()
    }

    pub fn shard_iter() -> impl CellStorage<Principal> {
        ShardIter::default()
    }

    pub fn ids() -> impl IDMap<Principal> {
        IDs::default()
    }
}
