use candid::Principal;
use catalyze_shared::{
    profile::{Profile, ProfileEntry, ProfileFilter},
    CanisterResult,
};
use common::{is_storage_index, CellStorage, ShardController};
use ic_cdk::{query, update};

use crate::{controller::ProfileController, storage::Index};

fn is_index_guard() -> Result<(), String> {
    is_storage_index(
        Index::default()
            .get()
            .expect("Failed to get index canister id"),
    )
}

#[query(guard = "is_index_guard")]
fn get(key: Principal) -> CanisterResult<ProfileEntry> {
    ProfileController.get(key)
}

#[query(guard = "is_index_guard")]
fn get_many(keys: Vec<Principal>) -> CanisterResult<Vec<ProfileEntry>> {
    ProfileController.get_many(keys)
}

#[query(guard = "is_index_guard")]
fn get_all() -> CanisterResult<Vec<ProfileEntry>> {
    ProfileController.get_all()
}

#[query(guard = "is_index_guard")]
fn find(filters: Vec<ProfileFilter>) -> CanisterResult<Option<ProfileEntry>> {
    ProfileController.find(filters)
}

#[query(guard = "is_index_guard")]
fn filter(filters: Vec<ProfileFilter>) -> CanisterResult<Vec<ProfileEntry>> {
    ProfileController.filter(filters)
}

#[update(guard = "is_index_guard")]
fn insert(key: Principal, value: Profile) -> CanisterResult<ProfileEntry> {
    ProfileController.insert(key, value)
}

#[update(guard = "is_index_guard")]
fn update(key: Principal, value: Profile) -> CanisterResult<ProfileEntry> {
    ProfileController.update(key, value)
}

#[update(guard = "is_index_guard")]
fn remove(key: Principal) -> CanisterResult<bool> {
    ProfileController.remove(key)
}

#[update(guard = "is_index_guard")]
fn remove_many(keys: Vec<Principal>) -> CanisterResult<()> {
    ProfileController.remove_many(keys)
}
