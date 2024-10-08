use std::{collections::HashMap, fmt::Display};

use async_trait::async_trait;
use candid::Principal;
use catalyze_shared::{
    api_error::ApiError, paged_response::PagedResponse, CanisterResult, CellStorage, Filter, Sorter,
};
use ic_cdk::api::management_canister::main::{
    install_code, CanisterInstallMode, InstallCodeArgument,
};
use ic_stable_structures::Storable;

use crate::{spawn_shard, IndexConfig, Registry, ShardClient, ShardsIndex};

#[async_trait]
pub trait IndexController<K, V, F, S>: Send + Sync
where
    K: 'static
        + candid::CandidType
        + for<'a> candid::Deserialize<'a>
        + std::hash::Hash
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
    F: 'static + Filter<K, V> + candid::CandidType + Clone + Send + Sync,
    S: 'static + Sorter<K, V> + Default + candid::CandidType + Clone + Send + Sync,
{
    /// Storage config
    fn config(&self) -> impl IndexConfig<K>;

    /// Shard client
    fn client(&self) -> impl ShardClient<K, V, F> {
        IndexShardClient
    }

    /// Upgrade shard wasm code by canister id
    async fn upgrade_shard(&self, canister_id: Principal) -> CanisterResult<()> {
        if !self.config().shards().get()?.contains(canister_id) {
            return Err(ApiError::not_found()
                .add_message(format!("Shard with the id {canister_id} not found")));
        }

        let wasm_module = self.config().shard_wasm().get()?;

        let install_args = InstallCodeArgument {
            mode: CanisterInstallMode::Upgrade(None),
            canister_id,
            wasm_module,
            arg: vec![],
        };

        install_code(install_args)
            .await
            .map_err(|(_, err)| ApiError::unexpected().add_message(err.as_str()))?;

        Ok(())
    }

    /// Extend the number of shards
    async fn extend_shards(&self, shards: u64) -> CanisterResult<ShardsIndex> {
        let shard_ids = self.config().shards().get().unwrap_or_default();
        let shard_wasm = self.config().shard_wasm().get()?;
        let mut new_shards_list = shard_ids.to_vec();

        for _ in 0..shards {
            new_shards_list.push(spawn_shard(shard_wasm.clone()).await?);
        }
        let shard_ids = self.config().shards().set(new_shards_list.clone().into())?;

        if self.config().shard_iter().get().is_err() {
            self.config().shard_iter().set(new_shards_list[0].id())?;
        }

        Ok(shard_ids)
    }

    /// Set the shard filled status
    fn set_shard_filled(&self, shard: Principal, filled: bool) -> CanisterResult<ShardsIndex> {
        let mut shard_ids = self.config().shards().get()?.to_vec();

        let idx = shard_ids
            .iter()
            .position(|s| s.id() == shard)
            .ok_or_else(|| {
                ApiError::not_found().add_message(format!("Shard with the id {shard} not found"))
            })?;

        let shard = shard_ids.get_mut(idx).expect("Shard not found");
        shard.set_filled(filled);

        self.config().shards().set(shard_ids.clone().into())
    }

    /// Get the next shard in the round-robin sequence
    fn next_shard(&self) -> CanisterResult<Principal> {
        let current = self.config().shard_iter().get()?;
        let shards = self
            .config()
            .shards()
            .get()?
            .to_vec()
            .into_iter()
            .filter(|s| !s.filled())
            .collect::<Vec<_>>();

        let current = shards
            .clone()
            .into_iter()
            .position(|x| x.id() == current)
            .unwrap_or_default();

        let next = if shards.len() == current + 1 {
            0
        } else {
            current + 1
        };

        self.config().shard_iter().set(shards[next].id())?;

        Ok(shards[current].id())
    }

    async fn size(&self) -> CanisterResult<u64> {
        let mut size = 0;
        let shards = self.config().shards().get()?;

        for shard in shards.to_vec().iter() {
            size += self.client().size(shard.id()).await?;
        }

        Ok(size)
    }

    async fn get(&self, id: K) -> CanisterResult<(K, V)> {
        let shard = self.config().registry().shard_by_id(id.clone())?;
        self.client().get(shard, id).await
    }

    async fn get_many(&self, ids: Vec<K>) -> CanisterResult<Vec<(K, V)>> {
        let mut res = Vec::new();

        let ids_map = ids
            .clone()
            .into_iter()
            .try_fold(HashMap::new(), |mut acc, id| {
                let shard = self.config().registry().shard_by_id(id.clone())?;
                let entry: &mut Vec<K> = acc.entry(shard).or_default();
                entry.push(id);
                Ok(acc)
            })?;

        for (shard, ids) in ids_map.into_iter() {
            let values = self.client().get_many(shard, ids).await?;
            res.extend(values);
        }

        // Sort result according to the key order
        let entries_map = res.into_iter().collect::<HashMap<K, V>>();

        Ok(ids
            .into_iter()
            .map(|id| {
                (
                    id.clone(),
                    entries_map.get(&id).cloned().expect("Entry not found"),
                )
            })
            .collect())
    }

    async fn get_all(&self) -> CanisterResult<Vec<(K, V)>> {
        let mut res = Vec::new();
        let shards = self.config().shards().get()?;

        for shard in shards.to_vec().iter() {
            let values = self.client().get_all(shard.id()).await?;
            res.extend(values);
        }

        Ok(S::default().sort(res))
    }

    async fn get_paginated(
        &self,
        limit: usize,
        page: usize,
        sort: S,
    ) -> CanisterResult<PagedResponse<(K, V)>> {
        let mut res = Vec::new();
        let shards = self.config().shards().get()?;

        for shard in shards.to_vec().iter() {
            let values = self.client().get_all(shard.id()).await?;
            res.extend(values);
        }

        Ok(PagedResponse::new(page, limit, sort.sort(res)))
    }

    async fn find(&self, filters: Vec<F>) -> CanisterResult<Option<(K, V)>> {
        let shards = self.config().shards().get()?;

        for shard in shards.to_vec().iter() {
            let value = self.client().find(shard.id(), filters.clone()).await?;
            if value.is_some() {
                return Ok(value);
            }
        }

        Ok(None)
    }

    async fn filter(&self, filters: Vec<F>) -> CanisterResult<Vec<(K, V)>> {
        let mut res = Vec::new();
        let shards = self.config().shards().get()?;

        for shard in shards.to_vec().iter() {
            let values = self.client().filter(shard.id(), filters.clone()).await?;
            res.extend(values);
        }

        Ok(S::default().sort(res))
    }

    async fn filter_paginated(
        &self,
        limit: usize,
        page: usize,
        sort: S,
        filters: Vec<F>,
    ) -> CanisterResult<PagedResponse<(K, V)>> {
        let mut res = Vec::new();
        let shards = self.config().shards().get()?;

        for shard in shards.to_vec().iter() {
            let values = self.client().filter(shard.id(), filters.clone()).await?;
            res.extend(values);
        }

        Ok(PagedResponse::new(page, limit, sort.sort(res)))
    }

    async fn insert(&self, key: K, value: V) -> CanisterResult<(K, V)> {
        if self.config().registry().exists(key.clone()) {
            return Err(ApiError::unexpected()
                .add_message("Key already exists")
                .add_info(key.to_string().as_str()));
        }

        let shard = self.next_shard()?;
        self.config().registry().insert(key.clone(), shard)?;
        self.client().insert(shard, key, value).await
    }

    async fn insert_many(&self, list: Vec<(K, V)>) -> CanisterResult<Vec<(K, V)>> {
        let list_map = list.clone().into_iter().collect::<HashMap<_, _>>();

        let ids_map = list
            .clone()
            .into_iter()
            .try_fold(HashMap::new(), |mut acc, (id, _)| {
                let shard = self.next_shard()?;
                let entry: &mut Vec<K> = acc.entry(shard).or_default();
                entry.push(id);
                Ok(acc)
            })?;

        for (shard, ids) in ids_map.into_iter() {
            let list = ids
                .into_iter()
                .map(|id| (id.clone(), list_map.get(&id).unwrap().clone()))
                .collect();

            self.client().insert_many(shard, list).await?;
        }

        Ok(list)
    }

    async fn update(&self, key: K, value: V) -> CanisterResult<(K, V)> {
        let shard = self.config().registry().shard_by_id(key.clone())?;
        self.client().update(shard, key, value).await
    }

    async fn update_many(&self, list: Vec<(K, V)>) -> CanisterResult<Vec<(K, V)>> {
        let list_map = list.clone().into_iter().collect::<HashMap<_, _>>();

        let ids_map = list
            .clone()
            .into_iter()
            .try_fold(HashMap::new(), |mut acc, (id, _)| {
                let shard = self.config().registry().shard_by_id(id.clone())?;
                let entry: &mut Vec<K> = acc.entry(shard).or_default();
                entry.push(id);
                Ok(acc)
            })?;

        let mut result = vec![];

        for (shard, ids) in ids_map.into_iter() {
            let list = ids
                .into_iter()
                .map(|id| (id.clone(), list_map.get(&id).unwrap().clone()))
                .collect();

            let mut updated = self.client().update_many(shard, list).await?;
            result.append(&mut updated);
        }

        // Sort result according to the key order
        let result_map = result.into_iter().collect::<HashMap<K, V>>();
        let sorted = list
            .into_iter()
            .map(|(id, _)| (id.clone(), result_map.get(&id).unwrap().clone()))
            .collect();

        Ok(sorted)
    }

    async fn remove(&self, key: K) -> CanisterResult<bool> {
        let shard = self.config().registry().shard_by_id(key.clone())?;
        self.client().remove(shard, key).await
    }

    async fn remove_many(&self, keys: Vec<K>) -> CanisterResult<()> {
        let ids_map = keys.into_iter().try_fold(HashMap::new(), |mut acc, id| {
            let shard = self.config().registry().shard_by_id(id.clone())?;
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

#[derive(Default)]
pub(crate) struct IndexShardClient;

impl<K, V, F> ShardClient<K, V, F> for IndexShardClient
where
    K: 'static
        + candid::CandidType
        + for<'a> candid::Deserialize<'a>
        + std::hash::Hash
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
    F: 'static + Filter<K, V> + candid::CandidType + Clone + Send + Sync,
{
}
