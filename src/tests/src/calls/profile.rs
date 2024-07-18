use candid::{Encode, Principal};
use catalyze_shared::profile::{Profile, ProfileEntry, ProfileFilter};

use crate::{result::CanisterResult, utils::Context};

use super::utils;

pub async fn insert(ctx: &Context, input: (Principal, Profile)) -> eyre::Result<ProfileEntry> {
    let resp = utils::update(
        ctx,
        &ctx.index_profile,
        "insert",
        Encode!(&input.0, &input.1)?,
    )
    .await?;
    CanisterResult::try_from(resp.as_slice())?.into_result()
}

pub async fn get(ctx: &Context, id: Principal) -> eyre::Result<ProfileEntry> {
    let resp = utils::query(ctx, &ctx.index_profile, "get", Encode!(&id)?).await?;
    CanisterResult::try_from(resp.as_slice())?.into_result()
}

pub async fn get_all(ctx: &Context) -> eyre::Result<Vec<ProfileEntry>> {
    let resp = utils::query(ctx, &ctx.index_profile, "get_all", Encode!()?).await?;
    CanisterResult::try_from(resp.as_slice())?.into_result()
}

pub async fn get_many(ctx: &Context, ids: Vec<Principal>) -> eyre::Result<Vec<ProfileEntry>> {
    let resp = utils::query(ctx, &ctx.index_profile, "get_many", Encode!(&ids)?).await?;
    CanisterResult::try_from(resp.as_slice())?.into_result()
}

pub async fn find(
    ctx: &Context,
    filters: Vec<ProfileFilter>,
) -> eyre::Result<Option<ProfileEntry>> {
    let resp = utils::query(ctx, &ctx.index_profile, "find", Encode!(&filters)?).await?;
    CanisterResult::try_from(resp.as_slice())?.into_result()
}

pub async fn filter(ctx: &Context, filters: Vec<ProfileFilter>) -> eyre::Result<Vec<ProfileEntry>> {
    let resp = utils::query(ctx, &ctx.index_profile, "filter", Encode!(&filters)?).await?;
    CanisterResult::try_from(resp.as_slice())?.into_result()
}

pub async fn update(ctx: &Context, input: (Principal, Profile)) -> eyre::Result<ProfileEntry> {
    let resp = utils::update(
        ctx,
        &ctx.index_profile,
        "update",
        Encode!(&input.0, &input.1)?,
    )
    .await?;
    CanisterResult::try_from(resp.as_slice())?.into_result()
}

pub async fn remove(ctx: &Context, key: Principal) -> eyre::Result<bool> {
    let resp = utils::update(ctx, &ctx.index_profile, "remove", Encode!(&key)?).await?;
    CanisterResult::try_from(resp.as_slice())?.into_result()
}

pub async fn remove_many(ctx: &Context, keys: Vec<Principal>) -> eyre::Result<()> {
    let resp = utils::update(ctx, &ctx.index_profile, "remove_many", Encode!(&keys)?).await?;
    CanisterResult::try_from(resp.as_slice())?.into_result()
}
