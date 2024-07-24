use candid::Principal;
use eyre::Context as _;

use crate::utils::Context;

pub async fn query(
    ctx: &Context,
    canister_id: &Principal,
    method: &str,
    args: Vec<u8>,
) -> eyre::Result<Vec<u8>> {
    let response = ctx
        .agent
        .query(canister_id, method)
        .with_arg(args)
        .await
        .wrap_err_with(|| format!("Failed to perform query \"{}\" request", method))?;

    Ok(response)
}

pub async fn update(
    ctx: &Context,
    canister_id: &Principal,
    method: &str,
    args: Vec<u8>,
) -> eyre::Result<Vec<u8>> {
    let response = ctx
        .agent
        .update(canister_id, method)
        .with_arg(args)
        .await
        .wrap_err_with(|| format!("Failed to perform update \"{}\" request", method))?;

    Ok(response)
}
