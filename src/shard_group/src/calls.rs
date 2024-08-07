use catalyze_shared::CanisterResult;
use common::{is_developer, is_storage_index, CellStorage, ShardController};
use ic_cdk::{query, update};

use crate::{
    aliases::{Entry, EntryFilter, Key, Value},
    controller::controller,
};

fn is_index_guard() -> Result<(), String> {
    if is_developer().is_ok() {
        return Ok(());
    }

    is_storage_index(
        controller()
            .index()
            .get()
            .expect("Failed to get index canister id"),
    )
}

#[query(guard = "is_index_guard")]
fn size() -> CanisterResult<u64> {
    controller().size()
}

#[query(guard = "is_index_guard")]
fn get(key: Key) -> CanisterResult<Entry> {
    controller().get(key)
}

#[query(guard = "is_index_guard")]
fn get_many(keys: Vec<Key>) -> CanisterResult<Vec<Entry>> {
    controller().get_many(keys)
}

#[query(guard = "is_index_guard")]
fn get_all() -> CanisterResult<Vec<Entry>> {
    controller().get_all()
}

#[query(guard = "is_index_guard")]
fn find(filters: Vec<EntryFilter>) -> CanisterResult<Option<Entry>> {
    controller().find(filters)
}

#[query(guard = "is_index_guard")]
fn filter(filters: Vec<EntryFilter>) -> CanisterResult<Vec<Entry>> {
    controller().filter(filters)
}

#[update(guard = "is_index_guard")]
fn insert(key: Key, value: Value) -> CanisterResult<Entry> {
    controller().insert(key, value)
}

#[update(guard = "is_index_guard")]
fn update(key: Key, value: Value) -> CanisterResult<Entry> {
    controller().update(key, value)
}

#[update(guard = "is_index_guard")]
fn update_many(list: Vec<Entry>) -> CanisterResult<Vec<Entry>> {
    controller().update_many(list)
}

#[update(guard = "is_index_guard")]
fn remove(key: Key) -> CanisterResult<bool> {
    controller().remove(key)
}

#[update(guard = "is_index_guard")]
fn remove_many(keys: Vec<Key>) -> CanisterResult<()> {
    controller().remove_many(keys)
}
