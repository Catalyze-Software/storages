use candid::Principal;
use catalyze_shared::CanisterResult;
use common::{queries, CellStorage, Principals};
use ic_cdk::{init, query, trap, update};
use storage::Storages;

mod storage;

#[query]
fn icts_name() -> String {
    queries::icts_name()
}

#[query]
fn icts_version() -> String {
    queries::icts_version()
}

#[init]
fn init(proxies: Vec<Principal>, _shards: u64) {
    if proxies.is_empty() {
        trap("Proxies cannot be empty");
    }

    Storages::proxies()
        .set(proxies.into())
        .expect("Failed to set proxies");

    // TODO: add deploy logic
    let shards = vec![];

    Storages::shards()
        .set(shards.clone().into())
        .expect("Failed to set shards");

    Storages::shard_iter()
        .set(shards[0])
        .expect("Failed to set shard iter");
}

#[update]
fn extend_shards(_shards: u64) -> CanisterResult<Principals> {
    let mut shards = Storages::shards().get()?;
    // TODO: add deploy logic
    shards.append(&mut vec![]);
    Storages::shards().set(shards)
}

// Hacky way to expose the candid interface to the outside world
#[query(name = "__get_candid_interface_tmp_hack")]
pub fn __export_did_tmp_() -> String {
    use candid::export_service;
    export_service!();
    __export_service()
}

// Method used to save the candid interface to a file
#[test]
pub fn candid() {
    catalyze_shared::candid::save_candid_file(
        "../../candid/index_profile.did",
        __export_did_tmp_(),
    );
}
