use common::{queries, CellStorage};
use ic_cdk::{caller, init, query};
use storage::Index;

mod calls;
mod controller;
mod storage;

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
    use candid::{export_service, Principal};
    use catalyze_shared::{
        profile::{Profile, ProfileEntry, ProfileFilter},
        CanisterResult,
    };
    export_service!();
    __export_service()
}

#[init]
fn init() {
    Index::default()
        .set(caller())
        .expect("Failed to set index canister ID");
}

// Method used to save the candid interface to a file
#[test]
pub fn candid() {
    catalyze_shared::candid::save_candid_file(
        "../../candid/shard_profile.did",
        __export_did_tmp_(),
    );
}
