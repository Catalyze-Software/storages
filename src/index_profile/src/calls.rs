use candid::Principal;
use catalyze_shared::{
    api_error::ApiError,
    profile::{Profile, ProfileEntry, ProfileFilter},
    CanisterResult,
};
use common::{is_developer, is_proxy, spawn_shard, CellStorage, IndexController, ShardsIndex};
use ic_cdk::{caller, init, query, trap, update};
use serde_bytes::ByteBuf;

use crate::{
    index::ProfileIndex,
    storage::{Proxies, ShardIter, ShardWasm, Shards},
};

fn is_proxy_guard() -> Result<(), String> {
    if is_developer().is_ok() {
        return Ok(());
    }

    is_proxy(Proxies::default().get().expect("Failed to get proxies"))
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

#[update(guard = "is_proxy_guard")]
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

#[update(guard = "is_proxy_guard")]
fn _dev_upload_wasm(wasm: ByteBuf) -> bool {
    ShardWasm::default().set(wasm.into_vec()).is_ok()
}

#[update(guard = "is_proxy_guard")]
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

#[query(composite = true, guard = "is_proxy_guard")]
async fn get(key: Principal) -> CanisterResult<ProfileEntry> {
    ProfileIndex.get(key).await
}

#[query(composite = true, guard = "is_proxy_guard")]
async fn get_many(keys: Vec<Principal>) -> CanisterResult<Vec<ProfileEntry>> {
    ProfileIndex.get_many(keys).await
}

#[query(composite = true, guard = "is_proxy_guard")]
async fn get_all() -> CanisterResult<Vec<ProfileEntry>> {
    ProfileIndex.get_all().await
}

#[query(composite = true, guard = "is_proxy_guard")]
async fn find(filters: Vec<ProfileFilter>) -> CanisterResult<Option<ProfileEntry>> {
    ProfileIndex.find(filters).await
}

#[query(composite = true, guard = "is_proxy_guard")]
async fn filter(filters: Vec<ProfileFilter>) -> CanisterResult<Vec<ProfileEntry>> {
    ProfileIndex.filter(filters).await
}

#[update(guard = "is_proxy_guard")]
async fn insert(key: Principal, value: Profile) -> CanisterResult<ProfileEntry> {
    ProfileIndex.insert(key, value).await
}

#[update(guard = "is_proxy_guard")]
async fn update(key: Principal, value: Profile) -> CanisterResult<ProfileEntry> {
    ProfileIndex.update(key, value).await
}

#[update(guard = "is_proxy_guard")]
async fn remove(key: Principal) -> CanisterResult<bool> {
    ProfileIndex.remove(key).await
}

#[update(guard = "is_proxy_guard")]
async fn remove_many(keys: Vec<Principal>) -> CanisterResult<()> {
    ProfileIndex.remove_many(keys).await
}
