use candid::{Encode, Principal};
use catalyze_shared::profile::{Profile, ProfileEntry};

use crate::{result::CanisterResult, utils::Context};

pub async fn insert(ctx: &Context, input: (Principal, Profile)) -> eyre::Result<ProfileEntry> {
    let args = Encode!(&input.0, &input.1).expect("Failed to encode arguments");
    let resp = ctx
        .agent
        .update(&ctx.index_profile, "insert")
        .with_arg(args)
        .await
        .expect("Failed to call insert");

    CanisterResult::try_from(resp.as_slice())
        .expect("failed to create result from profile insert call result")
        .into_result()
}
