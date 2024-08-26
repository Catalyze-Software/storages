use candid::Principal;
use catalyze_shared::CellStorage;
use common::queries;
use ic_cdk::{init, query, trap};

mod calls;
mod guards;
mod id;
mod state;

#[init]
pub fn init(proxies: Vec<Principal>) {
    if proxies.is_empty() {
        trap("No proxies provided");
    }

    state::proxies()
        .set(proxies.into())
        .expect("Failed to set proxies");
}

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
    use catalyze_shared::{reward::RewardableActivityResponse, CanisterResult};

    export_service!();
    __export_service()
}

// Method used to save the candid interface to a file
#[test]
pub fn candid() {
    catalyze_shared::candid::save_candid_file("../../candid/global.did", __export_did_tmp_());
}
