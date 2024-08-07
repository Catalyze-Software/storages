use candid::Principal;
use catalyze_shared::CanisterResult;
use common::{is_developer, is_storage_index, CellStorage, ShardController};
use ic_cdk::{query, update};

use crate::{
    aliases::{Entry, EntryFilter, Key, Value},
    controller::ProfileController,
    storage::Index,
};

fn is_index_guard() -> Result<(), String> {
    if is_developer().is_ok() {
        return Ok(());
    }

    is_storage_index(
        Index::default()
            .get()
            .expect("Failed to get index canister id"),
    )
}

#[query(guard = "is_index_guard")]
fn size() -> CanisterResult<u64> {
    ProfileController.size()
}

#[query(guard = "is_index_guard")]
fn get(key: Principal) -> CanisterResult<Entry> {
    ProfileController.get(key)
}

#[query(guard = "is_index_guard")]
fn get_many(keys: Vec<Key>) -> CanisterResult<Vec<Entry>> {
    ProfileController.get_many(keys)
}

#[query(guard = "is_index_guard")]
fn get_all() -> CanisterResult<Vec<Entry>> {
    ProfileController.get_all()
}

#[query(guard = "is_index_guard")]
fn find(filters: Vec<EntryFilter>) -> CanisterResult<Option<Entry>> {
    ProfileController.find(filters)
}

#[query(guard = "is_index_guard")]
fn filter(filters: Vec<EntryFilter>) -> CanisterResult<Vec<Entry>> {
    ProfileController.filter(filters)
}

#[update(guard = "is_index_guard")]
fn insert(key: Key, value: Value) -> CanisterResult<Entry> {
    ProfileController.insert(key, value)
}

#[update(guard = "is_index_guard")]
fn update(key: Key, value: Value) -> CanisterResult<Entry> {
    ProfileController.update(key, value)
}

#[update(guard = "is_index_guard")]
fn update_many(list: Vec<Entry>) -> CanisterResult<Vec<Entry>> {
    ProfileController.update_many(list)
}

#[update(guard = "is_index_guard")]
fn remove(key: Key) -> CanisterResult<bool> {
    ProfileController.remove(key)
}

#[update(guard = "is_index_guard")]
fn remove_many(keys: Vec<Key>) -> CanisterResult<()> {
    ProfileController.remove_many(keys)
}
