use std::time::Duration;

use candid::Principal;
use catalyze_shared::{
    api_error::ApiError,
    reward::{Activity, GroupReward, RewardDataPackage, UserActivity},
    CanisterResult, CellStorage, StorageClient,
};
use common::ShardStorage;
use ic_cdk::{api::time, call, spawn};
use ic_cdk_timers::set_timer_interval;

use crate::{
    clients::groups,
    logic::reward_buffer,
    state::{self, reward_canister, reward_timer},
};

pub const DAY_IN_SEC: u64 = 86400;

pub fn start() -> CanisterResult<()> {
    if reward_canister().is_empty() {
        return Err(ApiError::unexpected().add_message("Reward canister not set"));
    }

    let _ = set_timer_interval(Duration::from_secs(DAY_IN_SEC), move || {
        spawn(async move {
            let _ = send_reward_data().await;
        });
    });

    set_next_trigger()
}

pub fn next_trigger() -> CanisterResult<u64> {
    reward_timer().get()
}

pub fn set_next_trigger() -> CanisterResult<()> {
    reward_timer().set(time() + Duration::from_secs(DAY_IN_SEC).as_nanos() as u64)?;
    Ok(())
}

pub async fn send_reward_data() -> CanisterResult<()> {
    set_next_trigger()?;

    let reward_canister = reward_canister().get()?;
    let reward_data = process_buffer().await?;

    let _ = call::<(Vec<GroupReward>, Vec<UserActivity>, Vec<Principal>), ()>(
        reward_canister,
        "process_buffer",
        (
            reward_data.group_member_counts,
            reward_data.user_activity,
            reward_data.user_referrals,
        ),
    )
    .await;

    // clear buffer
    reward_buffer::clear();

    Ok(())
}

pub async fn process_buffer() -> CanisterResult<RewardDataPackage> {
    let rewardables = state::rewards_buffer().get_all();

    let mut user_activity = vec![];
    let mut group_ids = vec![];
    let mut user_referrals = vec![];

    for (_, rewardable) in rewardables.iter() {
        match rewardable.get_activity() {
            Activity::GroupMemberCount(id) => {
                group_ids.push(id);
            }
            Activity::UserActivity(principal) => {
                user_activity.push(UserActivity::new(principal, rewardable.get_timestamp()));
            }
            Activity::UserReferral(referrer) => {
                user_referrals.push(referrer);
            }
        }
    }

    let group_member_counts = groups()
        .get_many(group_ids)
        .await?
        .into_iter()
        // collect owner, group id and member count
        .map(|(id, group)| GroupReward::new(group.owner, id, group.get_members().len() as u64))
        .collect::<Vec<_>>();

    Ok(RewardDataPackage {
        group_member_counts,
        user_activity,
        user_referrals,
    })
}
