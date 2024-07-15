use candid::Principal;
use catalyze_shared::{api_error::ApiError, CanisterResult};
use ic_cdk::{
    api::{
        canister_balance,
        management_canister::main::{
            create_canister, install_code, CanisterInstallMode, CanisterSettings,
            CreateCanisterArgument, InstallCodeArgument,
        },
    },
    id,
};
use ic_ledger_types::{Tokens, MAINNET_CYCLES_MINTING_CANISTER_ID};

use crate::IcpXdrConversionRateResponse;

pub static MIN_CYCLES_FOR_SPINUP: u64 = 5_000_000_000_000;
pub static CATALYZE_E8S_FEE: Tokens = Tokens::from_e8s(10000000);

async fn get_minimum_spawn_icp_amount() -> CanisterResult<u64> {
    let result = ic_cdk::call::<(), (IcpXdrConversionRateResponse,)>(
        MAINNET_CYCLES_MINTING_CANISTER_ID,
        "get_icp_xdr_conversion_rate",
        (),
    )
    .await
    .map(|(rate,)| rate)
    .map_err(|_| ApiError::unexpected().add_message("Error getting XDR conversion rate"))?;

    let cycles_per_icp = result.data.xdr_permyriad_per_icp * 1_000_000_000_000 / 10_000;
    Ok(MIN_CYCLES_FOR_SPINUP / cycles_per_icp)
}

async fn spawn_canister(cycles: u64) -> CanisterResult<Principal> {
    let args = CreateCanisterArgument {
        settings: Some(CanisterSettings {
            controllers: Some(vec![id()]),
            compute_allocation: None,
            memory_allocation: None,
            freezing_threshold: None,
            reserved_cycles_limit: None,
            wasm_memory_limit: None,
        }),
    };

    create_canister(args, cycles as u128)
        .await
        .map(|(result,)| result.canister_id)
        .map_err(|(_, err)| ApiError::unexpected().add_message(err.as_str()))
}

pub async fn spawn_shard(wasm_module: Vec<u8>) -> CanisterResult<Principal> {
    let balance = canister_balance();
    let minimum_spawn_icp_amount = get_minimum_spawn_icp_amount().await?;

    if balance < minimum_spawn_icp_amount {
        return Err(ApiError::unexpected().add_message("Insufficient balance to spawn a shard"));
    }

    let shard_id = spawn_canister(minimum_spawn_icp_amount).await?;

    let install_args = InstallCodeArgument {
        mode: CanisterInstallMode::Install,
        canister_id: shard_id,
        wasm_module,
        arg: vec![],
    };

    install_code(install_args)
        .await
        .map_err(|(_, err)| ApiError::unexpected().add_message(err.as_str()))?;

    Ok(shard_id)
}
