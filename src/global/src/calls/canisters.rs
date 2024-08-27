use candid::Principal;
use catalyze_shared::{CanisterResult, CellStorage};
use common::is_developer;
use ic_cdk::{query, update};

use crate::state::{group_canister, reward_canister};

#[query(guard = "is_developer")]
fn _dev_get_reward_canister() -> CanisterResult<Principal> {
    reward_canister().get()
}

#[update(guard = "is_developer")]
fn _dev_set_reward_canister_id(canister_id: Principal) -> CanisterResult<Principal> {
    reward_canister().set(canister_id)
}

#[query(guard = "is_developer")]
fn _dev_get_group_canister() -> CanisterResult<Principal> {
    group_canister().get()
}

#[update(guard = "is_developer")]
fn _dev_set_group_canister_id(canister_id: Principal) -> CanisterResult<Principal> {
    group_canister().set(canister_id)
}
