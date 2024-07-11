use std::{collections::HashMap, fmt::Display};

use async_trait::async_trait;
use candid::Principal;
use catalyze_shared::{api_error::ApiError, CanisterResult};
use ic_stable_structures::Storable;

use crate::{CellStorage, Filter, IDMap, Principals, ShardClient, Sorter};

#[async_trait]
pub trait StorageIndex<K, V>: Send + Sync
where
    K: 'static
        + candid::CandidType
        + for<'a> candid::Deserialize<'a>
        + Storable
        + Ord
        + Clone
        + Display
        + Send
        + Sync,
    V: 'static
        + candid::CandidType
        + for<'a> candid::Deserialize<'a>
        + Storable
        + Clone
        + Send
        + Sync,
{
    /// Shards storage
    fn shards(&self) -> impl CellStorage<Principals>;
    /// Shard iterator storage, responsible for round-robin shard selection
    fn shard_iter(&self) -> impl CellStorage<Principal>;
    /// Entry ID to shard mapping storage
    fn ids(&self) -> impl IDMap<K>;
    /// Shard client
    fn client(&self) -> impl ShardClient<K, V>;
    /// Default sorter for values
    fn sorter(&self) -> impl Sorter<K, V>;

    /// Get the next shard in the round-robin sequence
    fn next_shard(&self) -> CanisterResult<Principal> {
        let current = self.shard_iter().get()?;
        let shards = self.shards().get()?.to_vec();
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

        self.shard_iter().set(shards[next])?;

        Ok(shards[current])
    }

    async fn get(&self, id: K) -> CanisterResult<(K, V)> {
        let shard = self.ids().shard_by_id(id.clone())?;
        self.client().get(shard, id).await
    }

    async fn get_many(&self, ids: Vec<K>) -> CanisterResult<Vec<(K, V)>> {
        let mut res = Vec::new();

        let ids_map = ids.into_iter().try_fold(HashMap::new(), |mut acc, id| {
            let shard = self.ids().shard_by_id(id.clone())?;
            let entry: &mut Vec<K> = acc.entry(shard).or_default();
            entry.push(id);
            Ok(acc)
        })?;

        for (shard, ids) in ids_map.into_iter() {
            let values = self.client().get_many(shard, ids).await?;
            res.extend(values);
        }

        Ok(self.sorter().sort(res))
    }

    async fn get_all(&self) -> CanisterResult<Vec<(K, V)>> {
        // TODO: pagination
        let mut res = Vec::new();
        let shards = self.shards().get()?;

        for shard in shards.to_vec().iter() {
            let values = self.client().get_all(*shard).await?;
            res.extend(values);
        }

        Ok(self.sorter().sort(res))
    }

    async fn find(&self, filters: Vec<impl Filter<V>>) -> CanisterResult<Option<(K, V)>> {
        let shards = self.shards().get()?;

        for shard in shards.to_vec().iter() {
            let value = self.client().find(*shard, filters.clone()).await?;
            if value.is_some() {
                return Ok(value);
            }
        }

        Ok(None)
    }

    async fn filter(&self, filters: Vec<impl Filter<V>>) -> CanisterResult<Vec<(K, V)>> {
        // TODO: pagination
        let mut res = Vec::new();
        let shards = self.shards().get()?;

        for shard in shards.to_vec().iter() {
            let values = self.client().filter(*shard, filters.clone()).await?;
            res.extend(values);
        }

        Ok(self.sorter().sort(res))
    }

    async fn insert(&self, key: K, value: V) -> CanisterResult<(K, V)> {
        if self.ids().exists(key.clone()) {
            return Err(ApiError::unexpected()
                .add_message("Key already exists")
                .add_info(key.to_string().as_str()));
        }

        let shard = self.next_shard()?;
        self.ids().insert(key.clone(), shard)?;
        self.client().insert(shard, key, value).await
    }

    async fn update(&self, key: K, value: V) -> CanisterResult<(K, V)> {
        let shard = self.ids().shard_by_id(key.clone())?;
        self.client().update(shard, key, value).await
    }

    async fn remove(&self, key: K) -> CanisterResult<bool> {
        let shard = self.ids().shard_by_id(key.clone())?;
        self.client().remove(shard, key).await
    }

    async fn remove_many(&self, keys: Vec<K>) -> CanisterResult<()> {
        let ids_map = keys.into_iter().try_fold(HashMap::new(), |mut acc, id| {
            let shard = self.ids().shard_by_id(id.clone())?;
            let entry: &mut Vec<K> = acc.entry(shard).or_default();
            entry.push(id);
            Ok(acc)
        })?;

        for (shard, ids) in ids_map.into_iter() {
            self.client().remove_many(shard, ids).await?;
        }

        Ok(())
    }
}
