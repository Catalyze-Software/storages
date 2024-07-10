use candid::Principal;
use catalyze_shared::{profile::Profile, CanisterResult};
use common::{is_proxy, CellStorage, StorageIndex};
use ic_cdk::update;

use crate::{index::ProfileIndex, storage::Proxies};

fn is_proxy_guard() -> Result<(), String> {
    is_proxy(Proxies::default().get().expect("Failed to get proxies"))
}

pub type ProfileEntry = (Principal, Profile);

#[update(guard = "is_proxy_guard")]
async fn get(key: Principal) -> CanisterResult<ProfileEntry> {
    ProfileIndex.get(key).await
}

#[update(guard = "is_proxy_guard")]
async fn get_many(keys: Vec<Principal>) -> CanisterResult<Vec<ProfileEntry>> {
    ProfileIndex.get_many(keys).await
}

#[update(guard = "is_proxy_guard")]
async fn get_all() -> CanisterResult<Vec<ProfileEntry>> {
    ProfileIndex.get_all().await
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
