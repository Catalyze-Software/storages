use candid::Principal;
use catalyze_shared::{api_error::ApiError, paged_response::PagedResponse, CanisterResult};
use common::{
    is_developer, is_proxy, spawn_shard, CellStorage, IndexConfig, IndexController, ShardsIndex,
};
use ic_cdk::{init, query, trap, update};
use serde_bytes::ByteBuf;

use crate::{
    aliases::{Entry, EntryFilter, EntrySort, Key, Value},
    config::config,
    index::index,
};

fn is_proxy_guard() -> Result<(), String> {
    if is_developer().is_ok() {
        return Ok(());
    }

    is_proxy(config().proxies().get().expect("Failed to get proxies"))
}

#[init]
fn init(proxies: Vec<Principal>) {
    if proxies.is_empty() {
        trap("Proxies cannot be empty");
    }

    config()
        .proxies()
        .set(proxies.into())
        .expect("Failed to set proxies");
}

#[update(guard = "is_proxy_guard")]
async fn _dev_extend_shards(shards: u64) -> CanisterResult<ShardsIndex> {
    let shard_ids = config().shards().get().unwrap_or_default();
    let shard_wasm = config().shard_wasm().get()?;
    let mut new_shards_list = shard_ids.to_vec();

    for _ in 0..shards {
        new_shards_list.push(spawn_shard(shard_wasm.clone()).await?);
    }
    let shard_ids = config().shards().set(new_shards_list.clone().into())?;

    if config().shard_iter().get().is_err() {
        config().shard_iter().set(new_shards_list[0].id())?;
    }

    Ok(shard_ids)
}

#[update(guard = "is_proxy_guard")]
fn _dev_upload_wasm(wasm: ByteBuf) -> bool {
    config().shard_wasm().set(wasm.into_vec()).is_ok()
}

#[update(guard = "is_proxy_guard")]
fn _dev_set_shard_filled(shard: Principal, filled: bool) -> CanisterResult<ShardsIndex> {
    let mut shard_ids = config().shards().get()?.to_vec();

    let idx = shard_ids
        .iter()
        .position(|s| s.id() == shard)
        .ok_or_else(|| {
            ApiError::not_found().add_message(format!("Shard with the id {shard} not found"))
        })?;

    let shard = shard_ids.get_mut(idx).expect("Shard not found");
    shard.set_filled(filled);

    config().shards().set(shard_ids.clone().into())
}

#[query(composite = true, guard = "is_proxy_guard")]
async fn size() -> CanisterResult<u64> {
    index().size().await
}

#[query(composite = true, guard = "is_proxy_guard")]
async fn get(key: Key) -> CanisterResult<Entry> {
    index().get(key).await
}

#[query(composite = true, guard = "is_proxy_guard")]
async fn get_many(keys: Vec<Key>) -> CanisterResult<Vec<Entry>> {
    index().get_many(keys).await
}

#[query(composite = true, guard = "is_proxy_guard")]
async fn get_all() -> CanisterResult<Vec<Entry>> {
    index().get_all().await
}

#[query(composite = true, guard = "is_proxy_guard")]
async fn get_paginated(
    limit: usize,
    page: usize,
    sort: EntrySort,
) -> CanisterResult<PagedResponse<Entry>> {
    index().get_paginated(limit, page, sort).await
}

#[query(composite = true, guard = "is_proxy_guard")]
async fn find(filters: Vec<EntryFilter>) -> CanisterResult<Option<Entry>> {
    index().find(filters).await
}

#[query(composite = true, guard = "is_proxy_guard")]
async fn filter(filters: Vec<EntryFilter>) -> CanisterResult<Vec<Entry>> {
    index().filter(filters).await
}

#[query(composite = true, guard = "is_proxy_guard")]
async fn filter_paginated(
    limit: usize,
    page: usize,
    sort: EntrySort,
    filters: Vec<EntryFilter>,
) -> CanisterResult<PagedResponse<Entry>> {
    index().filter_paginated(limit, page, sort, filters).await
}

#[update(guard = "is_proxy_guard")]
async fn insert(key: Key, value: Value) -> CanisterResult<Entry> {
    index().insert(key, value).await
}

#[update(guard = "is_proxy_guard")]
async fn update(key: Key, value: Value) -> CanisterResult<Entry> {
    index().update(key, value).await
}

#[update(guard = "is_proxy_guard")]
async fn update_many(list: Vec<Entry>) -> CanisterResult<Vec<Entry>> {
    index().update_many(list).await
}

#[update(guard = "is_proxy_guard")]
async fn remove(key: Key) -> CanisterResult<bool> {
    index().remove(key).await
}

#[update(guard = "is_proxy_guard")]
async fn remove_many(keys: Vec<Key>) -> CanisterResult<()> {
    index().remove_many(keys).await
}
