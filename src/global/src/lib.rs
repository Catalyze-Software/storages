use candid::Principal;
use catalyze_shared::CellStorage;
use common::queries;
use ic_cdk::{init, post_upgrade, query, trap};
use timers::reward_timer;

mod calls;
mod clients;
mod guards;
mod id;
mod logic;
mod state;
mod timers;

#[init]
pub fn init(proxies: Vec<Principal>) {
    if proxies.is_empty() {
        trap("No proxies provided");
    }

    state::proxies()
        .set(proxies.into())
        .expect("Failed to set proxies");

    if let Err(e) = reward_timer::start() {
        trap(&format!("Failed to start reward timer: {e}"));
    }
}

#[post_upgrade]
pub async fn post_upgrade() {
    if let Err(e) = reward_timer::start() {
        trap(&format!("Failed to start reward timer: {e}"));
    }
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
