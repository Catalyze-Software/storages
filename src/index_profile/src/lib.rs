use candid::Principal;
use catalyze_shared::CanisterResult;
use common::{queries, spawn_shard, CellStorage, Principals};
use ic_cdk::{init, query, trap, update};
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
async fn init(shard_wasm: Vec<u8>, proxies: Vec<Principal>, shards: u64) {
    if proxies.is_empty() {
        trap("Proxies cannot be empty");
    }

    Proxies::default()
        .set(proxies.into())
        .expect("Failed to set proxies");

    ShardWasm::default()
        .set(shard_wasm.clone())
        .expect("Failed to set shard wasm");

    let mut shard_ids = vec![];

    for _ in 0..shards {
        shard_ids.push(
            spawn_shard(shard_wasm.clone())
                .await
                .expect("Failed to spawn shard"),
        );
    }

    Shards::default()
        .set(shard_ids.clone().into())
        .expect("Failed to set shards");

    ShardIter::default()
        .set(shard_ids[0])
        .expect("Failed to set shard iter");
}

#[update]
async fn extend_shards(shards: u64) -> CanisterResult<Principals> {
    let shard_ids = Shards::default().get()?;
    let shard_wasm = ShardWasm::default().get()?;
    let mut new_shards_list = shard_ids.to_vec();

    for _ in 0..shards {
        new_shards_list.push(spawn_shard(shard_wasm.clone()).await?);
    }
    Shards::default().set(new_shards_list.into())
}

// Method used to save the candid interface to a file
#[test]
pub fn candid() {
    catalyze_shared::candid::save_candid_file(
        "../../candid/index_profile.did",
        __export_did_tmp_(),
    );
}
