use candid::Principal;
use catalyze_shared::{
    api_error::ApiError, paged_response::PagedResponse, CanisterResult, CellStorage,
};
use common::{
    controller, is_developer, is_migration, is_proxy, spawn_shard, IDIter, IndexConfig,
    IndexConfigBase, IndexConfigWithKeyIter, IndexController, Principals, ShardsIndex,
};
use ic_cdk::{init, query, trap, update};
use serde_bytes::ByteBuf;

use crate::{
    aliases::{Entry, EntryFilter, EntrySort, Key, Value},
    config::config,
    controller::controller,
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

    set_proxies(proxies).expect("Failed to set proxies");
}

#[update(guard = "is_proxy_guard")]
fn set_proxies(proxies: Vec<Principal>) -> CanisterResult<Principals> {
    config().proxies().set(proxies.into())
}

#[query(guard = "is_proxy_guard")]
fn _dev_get_shards() -> CanisterResult<ShardsIndex> {
    config().shards().get()
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
    controller().size().await
}

#[query(composite = true, guard = "is_proxy_guard")]
async fn get(key: Key) -> CanisterResult<Entry> {
    controller().get(key).await
}

#[query(composite = true, guard = "is_proxy_guard")]
async fn get_many(keys: Vec<Key>) -> CanisterResult<Vec<Entry>> {
    controller().get_many(keys).await
}

#[query(composite = true, guard = "is_proxy_guard")]
async fn get_all() -> CanisterResult<Vec<Entry>> {
    controller().get_all().await
}

#[query(composite = true, guard = "is_proxy_guard")]
async fn get_paginated(
    limit: usize,
    page: usize,
    sort: EntrySort,
) -> CanisterResult<PagedResponse<Entry>> {
    controller().get_paginated(limit, page, sort).await
}

#[query(composite = true, guard = "is_proxy_guard")]
async fn find(filters: Vec<EntryFilter>) -> CanisterResult<Option<Entry>> {
    controller().find(filters).await
}

#[query(composite = true, guard = "is_proxy_guard")]
async fn filter(filters: Vec<EntryFilter>) -> CanisterResult<Vec<Entry>> {
    controller().filter(filters).await
}

#[query(composite = true, guard = "is_proxy_guard")]
async fn filter_paginated(
    limit: usize,
    page: usize,
    sort: EntrySort,
    filters: Vec<EntryFilter>,
) -> CanisterResult<PagedResponse<Entry>> {
    controller()
        .filter_paginated(limit, page, sort, filters)
        .await
}

#[update(guard = "is_proxy_guard")]
async fn insert(value: Value) -> CanisterResult<Entry> {
    controller()
        .insert(config().key_iter().next()?, value)
        .await
}

#[update(guard = "is_migration")]
async fn insert_by_key(key: Key, value: Value) -> CanisterResult<Entry> {
    controller::insert_by_key(controller(), config().key_iter(), key, value).await
}

#[update(guard = "is_proxy_guard")]
async fn update(key: Key, value: Value) -> CanisterResult<Entry> {
    controller().update(key, value).await
}

#[update(guard = "is_proxy_guard")]
async fn update_many(list: Vec<Entry>) -> CanisterResult<Vec<Entry>> {
    controller().update_many(list).await
}

#[update(guard = "is_proxy_guard")]
async fn remove(key: Key) -> CanisterResult<bool> {
    controller().remove(key).await
}

#[update(guard = "is_proxy_guard")]
async fn remove_many(keys: Vec<Key>) -> CanisterResult<()> {
    controller().remove_many(keys).await
}
