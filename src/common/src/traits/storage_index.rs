use candid::Principal;
use catalyze_shared::{api_error::ApiError, CanisterResult};
use ic_stable_structures::Storable;

use crate::{CellStorage, IDMap, Principals, ShardClient};

pub trait StorageIndex<K, V>
where
    K: 'static + candid::CandidType + for<'a> candid::Deserialize<'a> + Storable + Ord + Clone,
    V: candid::CandidType + for<'a> candid::Deserialize<'a> + Storable + Clone,
{
    /// Shards storage
    fn shards() -> impl CellStorage<Principals>;
    /// Shard iterator storage, responsible for round-robin shard selection
    fn shard_iter() -> impl CellStorage<Principal>;
    /// Entry ID to shard mapping storage
    fn ids() -> impl IDMap<K>;
    /// Shard client
    fn client() -> impl ShardClient<K, V>;

    /// Get the next shard in the round-robin sequence
    fn next_shard() -> CanisterResult<Principal> {
        let current = Self::shard_iter().get()?;
        let shards = Self::shards().get()?.to_vec();
        let current = shards
            .clone()
            .into_iter()
            .position(|x| x == current)
            .ok_or_else(|| {
                ApiError::unexpected().add_message("Failed to find current shard in shards list")
            })?;

        let next = if shards.len() == current + 1 {
            0
        } else {
            current + 1
        };

        Self::shard_iter().set(shards[next])?;

        Ok(shards[current])
    }

    async fn get(id: K) -> CanisterResult<V> {
        let shard = Self::ids().shard_by_id(id.clone())?;
        Self::client().get(shard, id).await
    }
}
