use catalyze_shared::{
    reward::{RewardableActivity, RewardableActivityResponse},
    CanisterResult,
};
use common::ShardStorage;
use ic_stable_structures::StableBTreeMap;

use crate::{
    id,
    state::{self, MEMORY_MANAGER, REWARD_BUFFER, REWARD_BUFFER_MEMORY_ID},
};

pub fn read() -> Vec<RewardableActivityResponse> {
    state::rewards_buffer()
        .get_all()
        .into_iter()
        .map(|(_, v)| v.into())
        .collect()
}

pub fn clear() {
    REWARD_BUFFER.with(|n| {
        n.replace(StableBTreeMap::new(
            MEMORY_MANAGER.with(|m| m.borrow().get(REWARD_BUFFER_MEMORY_ID)),
        ))
    });
}

pub fn insert(activity: RewardableActivity) -> CanisterResult<()> {
    state::rewards_buffer().insert_by_key(id::next(id::IDKind::RewardBuffer)?, activity)?;
    Ok(())
}
