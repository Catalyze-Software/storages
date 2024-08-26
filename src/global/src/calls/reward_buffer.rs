use candid::Principal;
use catalyze_shared::{
    reward::{Activity, RewardableActivity, RewardableActivityResponse},
    CanisterResult,
};
use common::ShardStorage;
use ic_cdk::{query, update};

use crate::{
    guards::is_proxy,
    id::{self},
    state::{self},
};

#[query(guard = "is_proxy")]
fn read_reward_buffer() -> Vec<RewardableActivityResponse> {
    state::rewards_buffer()
        .get_all()
        .into_iter()
        .map(|(_, v)| v.into())
        .collect()
}

#[update(guard = "is_proxy")]
fn clear_reward_buffer() {
    state::clear_reward_buffer();
}

#[update(guard = "is_proxy")]
pub fn notify_group_member_count_changed(group_id: u64) -> CanisterResult<()> {
    let activity = RewardableActivity::new(Activity::GroupMemberCount(group_id));
    insert(activity)
}

#[update(guard = "is_proxy")]
pub fn notify_active_user(principal: Principal) -> CanisterResult<()> {
    let activity = RewardableActivity::new(Activity::UserActivity(principal));
    insert(activity)
}

fn insert(activity: RewardableActivity) -> CanisterResult<()> {
    state::rewards_buffer().insert_by_key(id::next(id::IDKind::RewardBuffer)?, activity)?;
    Ok(())
}
