use catalyze_shared::CellStorage;
use common::{queries, ShardController};
use ic_cdk::{caller, init, query};

mod aliases;
mod calls;
mod controller;
mod state;

#[query]
fn icts_name() -> String {
    queries::icts_name()
}

#[query]
fn icts_version() -> String {
    queries::icts_version()
}

// Hacky way to expose the candid interface to the outside world
#[query(name = "__get_candid_interface_tmp_hack")]
pub fn __export_did_tmp_() -> String {
    use crate::aliases::*;
    use candid::export_service;
    use catalyze_shared::CanisterResult;

    export_service!();
    __export_service()
}

#[init]
fn init() {
    controller::controller()
        .index()
        .set(caller())
        .expect("Failed to set index canister ID");
}

// Method used to save the candid interface to a file
#[test]
pub fn candid() {
    catalyze_shared::candid::save_candid_file(
        &format!("../../candid/shard_{}.did", crate::aliases::DATA_KIND),
        __export_did_tmp_(),
    );
}
