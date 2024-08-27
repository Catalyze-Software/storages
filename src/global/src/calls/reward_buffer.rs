use candid::Principal;
use catalyze_shared::{
    reward::{Activity, RewardableActivity, RewardableActivityResponse},
    CanisterResult,
};
use ic_cdk::{query, update};

use crate::{guards::is_proxy, logic::reward_buffer, timers::reward_timer};

#[query(guard = "is_proxy")]
fn read_reward_buffer() -> Vec<RewardableActivityResponse> {
    reward_buffer::read()
}

#[update(guard = "is_proxy")]
fn clear_reward_buffer() {
    reward_buffer::clear();
}

#[query]
fn reward_timer_next_trigger() -> CanisterResult<u64> {
    reward_timer::next_trigger()
}

#[update(guard = "is_proxy")]
pub fn notify_group_member_count_changed(group_id: u64) -> CanisterResult<()> {
    let activity = RewardableActivity::new(Activity::GroupMemberCount(group_id));
    reward_buffer::insert(activity)
}

#[update(guard = "is_proxy")]
pub fn notify_active_user(principal: Principal) -> CanisterResult<()> {
    let activity = RewardableActivity::new(Activity::UserActivity(principal));
    reward_buffer::insert(activity)
}
