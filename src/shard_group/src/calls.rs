use catalyze_shared::{
    group::{Group, GroupEntry, GroupFilter},
    CanisterResult,
};
use common::{is_developer, is_storage_index, CellStorage, ShardController};
use ic_cdk::{query, update};

use crate::{controller::GroupController, storage::Index};

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
    GroupController.size()
}

#[query(guard = "is_index_guard")]
fn get(key: u64) -> CanisterResult<GroupEntry> {
    GroupController.get(key)
}

#[query(guard = "is_index_guard")]
fn get_many(keys: Vec<u64>) -> CanisterResult<Vec<GroupEntry>> {
    GroupController.get_many(keys)
}

#[query(guard = "is_index_guard")]
fn get_all() -> CanisterResult<Vec<GroupEntry>> {
    GroupController.get_all()
}

#[query(guard = "is_index_guard")]
fn find(filters: Vec<GroupFilter>) -> CanisterResult<Option<GroupEntry>> {
    GroupController.find(filters)
}

#[query(guard = "is_index_guard")]
fn filter(filters: Vec<GroupFilter>) -> CanisterResult<Vec<GroupEntry>> {
    GroupController.filter(filters)
}

#[update(guard = "is_index_guard")]
fn insert(key: u64, value: Group) -> CanisterResult<GroupEntry> {
    GroupController.insert(key, value)
}

#[update(guard = "is_index_guard")]
fn update(key: u64, value: Group) -> CanisterResult<GroupEntry> {
    GroupController.update(key, value)
}

#[update(guard = "is_index_guard")]
fn update_many(list: Vec<GroupEntry>) -> CanisterResult<Vec<GroupEntry>> {
    GroupController.update_many(list)
}

#[update(guard = "is_index_guard")]
fn remove(key: u64) -> CanisterResult<bool> {
    GroupController.remove(key)
}

#[update(guard = "is_index_guard")]
fn remove_many(keys: Vec<u64>) -> CanisterResult<()> {
    GroupController.remove_many(keys)
}
