use candid::Principal;
use catalyze_shared::{paged_response::PagedResponse, CanisterResult, CellStorage};
use common::{
    controller, is_developer, is_migration, is_proxy, IndexConfigBase, IndexConfigWithKeyIter,
    IndexControllerStateful, Principals,
};
use ic_cdk::{init, post_upgrade, query, trap, update};

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

#[post_upgrade]
pub fn post_upgrade() {
    controller()
        .start_timers_after_upgrade()
        .expect("Failed to start timers after upgrade");
}

#[update(guard = "is_proxy_guard")]
fn set_proxies(proxies: Vec<Principal>) -> CanisterResult<Principals> {
    config().proxies().set(proxies.into())
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
    controller().new_boost(value)
}

#[update(guard = "is_proxy_guard")]
fn insert_many(list: Vec<Value>) -> CanisterResult<Vec<Entry>> {
    controller().new_boost_many(list)
}

#[update(guard = "is_migration")]
fn insert_by_key(key: Key, value: Value) -> CanisterResult<Entry> {
    let (key, value) =
        controller::insert_by_key_stateful(controller(), config().key_iter(), key, value)?;
    controller().set_timer(key, value.seconds);

    Ok((key, value))
}

#[update(guard = "is_proxy_guard")]
fn update(key: Key, value: Value) -> CanisterResult<Entry> {
    controller().update_boost(key, value)
}

#[update(guard = "is_proxy_guard")]
fn update_many(list: Vec<Entry>) -> CanisterResult<Vec<Entry>> {
    controller().update_boost_many(list)
}

#[update(guard = "is_proxy_guard")]
fn remove(key: Key) -> CanisterResult<bool> {
    controller().remove_boost(key)
}

#[update(guard = "is_proxy_guard")]
fn remove_many(keys: Vec<Key>) -> CanisterResult<()> {
    controller().remove_boost_many(keys)
}
