use candid::Principal;
use catalyze_shared::{api_error::ApiError, CanisterResult};
use common::{queries, spawn_shard, CellStorage, ShardsIndex};
use ic_cdk::{init, query, trap, update};
use serde_bytes::ByteBuf;
use storage::{Proxies, ShardIter, ShardWasm, Shards};

mod calls;
mod index;
mod storage;

#[query]
fn icts_name() -> String {
    queries::icts_name()
}

#[query]
fn icts_version() -> String {
    queries::icts_version()
}

// Hacky way to expose the candid interface to the outside world
#[query(name = "__get_candid_interface_tmp_hack")]
pub fn __export_did_tmp_() -> String {
    use candid::export_service;
    use catalyze_shared::profile::{Profile, ProfileEntry, ProfileFilter};

    export_service!();
    __export_service()
}

#[init]
async fn init(proxies: Vec<Principal>) {
    if proxies.is_empty() {
        trap("Proxies cannot be empty");
    }

    Proxies::default()
        .set(proxies.into())
        .expect("Failed to set proxies");
}

// TODO: Add guard dev
#[update]
async fn _dev_extend_shards(shards: u64) -> CanisterResult<ShardsIndex> {
    let shard_ids = Shards::default().get().unwrap_or_default();
    let shard_wasm = ShardWasm::default().get()?;
    let mut new_shards_list = shard_ids.to_vec();

    for _ in 0..shards {
        new_shards_list.push(spawn_shard(shard_wasm.clone()).await?);
    }
    let shard_ids = Shards::default().set(new_shards_list.clone().into())?;

    if ShardIter::default().get().is_err() {
        ShardIter::default().set(new_shards_list[0].id())?;
    }

    Ok(shard_ids)
}

// TODO: Add guard dev
#[update]
fn _dev_upload_wasm(wasm: ByteBuf) -> bool {
    ShardWasm::default().set(wasm.into_vec()).is_ok()
}

// TODO: Add guard dev
#[update]
fn _dev_set_shard_filled(shard: Principal, filled: bool) -> CanisterResult<ShardsIndex> {
    let mut shard_ids = Shards::default().get()?.to_vec();

    let idx = shard_ids
        .iter()
        .position(|s| s.id() == shard)
        .ok_or_else(|| {
            ApiError::not_found().add_message(format!("Shard with the id {shard} not found"))
        })?;

    let shard = shard_ids.get_mut(idx).expect("Shard not found");
    shard.set_filled(filled);

    Shards::default().set(shard_ids.clone().into())
}

// Method used to save the candid interface to a file
#[test]
pub fn candid() {
    catalyze_shared::candid::save_candid_file(
        "../../candid/index_profile.did",
        __export_did_tmp_(),
    );
}
