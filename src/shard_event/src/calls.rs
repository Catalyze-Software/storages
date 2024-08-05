use catalyze_shared::{
    event::{Event, EventEntry, EventFilter},
    CanisterResult,
};
use common::{is_developer, is_storage_index, CellStorage, ShardController};
use ic_cdk::{query, update};

use crate::{controller::EventController, storage::Index};

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
    EventController.size()
}

#[query(guard = "is_index_guard")]
fn get(key: u64) -> CanisterResult<EventEntry> {
    EventController.get(key)
}

#[query(guard = "is_index_guard")]
fn get_many(keys: Vec<u64>) -> CanisterResult<Vec<EventEntry>> {
    EventController.get_many(keys)
}

#[query(guard = "is_index_guard")]
fn get_all() -> CanisterResult<Vec<EventEntry>> {
    EventController.get_all()
}

#[query(guard = "is_index_guard")]
fn find(filters: Vec<EventFilter>) -> CanisterResult<Option<EventEntry>> {
    EventController.find(filters)
}

#[query(guard = "is_index_guard")]
fn filter(filters: Vec<EventFilter>) -> CanisterResult<Vec<EventEntry>> {
    EventController.filter(filters)
}

#[update(guard = "is_index_guard")]
fn insert(key: u64, value: Event) -> CanisterResult<EventEntry> {
    EventController.insert(key, value)
}

#[update(guard = "is_index_guard")]
fn update(key: u64, value: Event) -> CanisterResult<EventEntry> {
    EventController.update(key, value)
}

#[update(guard = "is_index_guard")]
fn update_many(list: Vec<EventEntry>) -> CanisterResult<Vec<EventEntry>> {
    EventController.update_many(list)
}

#[update(guard = "is_index_guard")]
fn remove(key: u64) -> CanisterResult<bool> {
    EventController.remove(key)
}

#[update(guard = "is_index_guard")]
fn remove_many(keys: Vec<u64>) -> CanisterResult<()> {
    EventController.remove_many(keys)
}
