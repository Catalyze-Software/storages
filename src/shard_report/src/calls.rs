use catalyze_shared::{
    report::{Report, ReportEntry, ReportFilter},
    CanisterResult,
};
use common::{is_developer, is_storage_index, CellStorage, ShardController};
use ic_cdk::{query, update};

use crate::{controller::ReportController, storage::Index};

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
    ReportController.size()
}

#[query(guard = "is_index_guard")]
fn get(key: u64) -> CanisterResult<ReportEntry> {
    ReportController.get(key)
}

#[query(guard = "is_index_guard")]
fn get_many(keys: Vec<u64>) -> CanisterResult<Vec<ReportEntry>> {
    ReportController.get_many(keys)
}

#[query(guard = "is_index_guard")]
fn get_all() -> CanisterResult<Vec<ReportEntry>> {
    ReportController.get_all()
}

#[query(guard = "is_index_guard")]
fn find(filters: Vec<ReportFilter>) -> CanisterResult<Option<ReportEntry>> {
    ReportController.find(filters)
}

#[query(guard = "is_index_guard")]
fn filter(filters: Vec<ReportFilter>) -> CanisterResult<Vec<ReportEntry>> {
    ReportController.filter(filters)
}

#[update(guard = "is_index_guard")]
fn insert(key: u64, value: Report) -> CanisterResult<ReportEntry> {
    ReportController.insert(key, value)
}

#[update(guard = "is_index_guard")]
fn update(key: u64, value: Report) -> CanisterResult<ReportEntry> {
    ReportController.update(key, value)
}

#[update(guard = "is_index_guard")]
fn update_many(list: Vec<ReportEntry>) -> CanisterResult<Vec<ReportEntry>> {
    ReportController.update_many(list)
}

#[update(guard = "is_index_guard")]
fn remove(key: u64) -> CanisterResult<bool> {
    ReportController.remove(key)
}

#[update(guard = "is_index_guard")]
fn remove_many(keys: Vec<u64>) -> CanisterResult<()> {
    ReportController.remove_many(keys)
}
