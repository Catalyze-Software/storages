use common::ShardStorage;
use ic_cdk::query;

use crate::{guards::is_proxy, state};

mod history_point;
mod reward_buffer;

#[query(guard = "is_proxy")]
fn get_all_ids() -> Vec<(String, u64)> {
    state::id_storage().get_all()
}
