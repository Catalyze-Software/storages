use candid::Principal;
use catalyze_shared::{CanisterResult, CellStorage};
use common::is_developer;
use ic_cdk::{query, update};

use crate::state::reward_canister;

#[query]
fn get_reward_canister_id() -> CanisterResult<Principal> {
    reward_canister().get()
}

#[update(guard = "is_developer")]
fn set_reward_canister_id(canister_id: Principal) -> CanisterResult<Principal> {
    reward_canister().set(canister_id)
}
