use candid::Principal;
use catalyze_shared::{paged_response::PagedResponse, CanisterResult, CellStorage};
use common::{
    controller, is_developer, is_migration, is_proxy, IDIter, IndexConfig, IndexConfigBase,
    IndexConfigWithKeyIter, IndexController, Principals, ShardsIndex,
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
    controller().extend_shards(shards).await
}

#[update(guard = "is_proxy_guard")]
fn _dev_upload_wasm(wasm: ByteBuf) -> bool {
    config().shard_wasm().set(wasm.into_vec()).is_ok()
}

#[update(guard = "is_proxy_guard")]
fn _dev_set_shard_filled(shard: Principal, filled: bool) -> CanisterResult<ShardsIndex> {
    controller().set_shard_filled(shard, filled)
}

#[update(guard = "is_proxy_guard")]
async fn _dev_upgrade_shard(shard: Principal) -> CanisterResult<()> {
    controller().upgrade_shard(shard).await
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
