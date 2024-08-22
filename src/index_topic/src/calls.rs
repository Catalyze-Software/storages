use candid::Principal;
use catalyze_shared::{paged_response::PagedResponse, CanisterResult, CellStorage};
use common::{
    controller, is_developer, is_migration, is_proxy, IDIter, IndexConfigBase,
    IndexConfigWithKeyIter, IndexControllerStateful,
};
use ic_cdk::{init, query, trap, update};

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

    config()
        .proxies()
        .set(proxies.into())
        .expect("Failed to set proxies");
}

#[query(guard = "is_proxy_guard")]
fn size() -> CanisterResult<u64> {
    controller().size()
}

#[query(guard = "is_proxy_guard")]
fn get(key: Key) -> CanisterResult<Entry> {
    controller().get(key)
}

#[query(guard = "is_proxy_guard")]
fn get_many(keys: Vec<Key>) -> CanisterResult<Vec<Entry>> {
    controller().get_many(keys)
}

#[query(guard = "is_proxy_guard")]
fn get_all() -> CanisterResult<Vec<Entry>> {
    controller().get_all()
}

#[query(guard = "is_proxy_guard")]
fn get_paginated(
    limit: usize,
    page: usize,
    sort: EntrySort,
) -> CanisterResult<PagedResponse<Entry>> {
    controller().get_paginated(limit, page, sort)
}

#[query(guard = "is_proxy_guard")]
fn find(filters: Vec<EntryFilter>) -> CanisterResult<Option<Entry>> {
    controller().find(filters)
}

#[query(guard = "is_proxy_guard")]
fn filter(filters: Vec<EntryFilter>) -> CanisterResult<Vec<Entry>> {
    controller().filter(filters)
}

#[query(guard = "is_proxy_guard")]
fn filter_paginated(
    limit: usize,
    page: usize,
    sort: EntrySort,
    filters: Vec<EntryFilter>,
) -> CanisterResult<PagedResponse<Entry>> {
    controller().filter_paginated(limit, page, sort, filters)
}

#[update(guard = "is_proxy_guard")]
fn insert(value: Value) -> CanisterResult<Entry> {
    controller().insert(config().key_iter().next()?, value)
}

#[update(guard = "is_migration")]
fn insert_by_key(key: Key, value: Value) -> CanisterResult<Entry> {
    controller::insert_by_key_stateful(controller(), config().key_iter(), key, value)
}

#[update(guard = "is_proxy_guard")]
fn update(key: Key, value: Value) -> CanisterResult<Entry> {
    controller().update(key, value)
}

#[update(guard = "is_proxy_guard")]
fn update_many(list: Vec<Entry>) -> CanisterResult<Vec<Entry>> {
    controller().update_many(list)
}

#[update(guard = "is_proxy_guard")]
fn remove(key: Key) -> CanisterResult<bool> {
    controller().remove(key)
}

#[update(guard = "is_proxy_guard")]
fn remove_many(keys: Vec<Key>) -> CanisterResult<()> {
    controller().remove_many(keys)
}
