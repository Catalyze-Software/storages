use catalyze_shared::{CanisterResult, CellStorage};
use ic_cdk::{query, update};

use crate::{guards::is_proxy, state};

#[query(guard = "is_proxy")]
fn get_history_point() -> CanisterResult<u64> {
    state::history_point().get()
}

#[update(guard = "is_proxy")]
pub fn next_history_point() -> CanisterResult<u64> {
    let current = state::history_point().get().unwrap_or(1);
    let next = current + 1;
    state::history_point().set(next)?;
    Ok(next)
}
