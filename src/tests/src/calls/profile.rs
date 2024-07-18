use candid::{Encode, Principal};
use catalyze_shared::profile::{Profile, ProfileEntry};

use crate::{result::CanisterResult, utils::Context};

pub async fn insert(ctx: &Context, input: (Principal, Profile)) -> eyre::Result<ProfileEntry> {
    let args = Encode!(&input.0, &input.1).expect("Failed to encode insert arguments");
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

pub async fn get(ctx: &Context, id: Principal) -> eyre::Result<ProfileEntry> {
    let args = Encode!(&id).expect("Failed to encode get arguments");
    let resp = ctx
        .agent
        .query(&ctx.index_profile, "get")
        .with_arg(args)
        .await
        .expect("Failed to call get");

    CanisterResult::try_from(resp.as_slice())
        .expect("failed to create result from profile get call result")
        .into_result()
}

pub async fn get_all(ctx: &Context) -> eyre::Result<Vec<ProfileEntry>> {
    let resp = ctx
        .agent
        .query(&ctx.index_profile, "get_all")
        .await
        .expect("Failed to call get all");

    CanisterResult::try_from(resp.as_slice())
        .expect("failed to create result from profile get all call result")
        .into_result()
}

pub async fn get_many(ctx: &Context, ids: Vec<Principal>) -> eyre::Result<Vec<ProfileEntry>> {
    let args = Encode!(&ids).expect("Failed to encode get many arguments");
    let resp = ctx
        .agent
        .query(&ctx.index_profile, "get_many")
        .with_arg(args)
        .await
        .expect("Failed to call get many");

    CanisterResult::try_from(resp.as_slice())
        .expect("failed to create result from profile get many call result")
        .into_result()
}
