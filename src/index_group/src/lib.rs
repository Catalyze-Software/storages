use candid::Principal;
use catalyze_shared::CanisterResult;
use common::{queries, ShardsIndex};
use ic_cdk::query;
use serde_bytes::ByteBuf;

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
    use catalyze_shared::group::{Group, GroupEntry, GroupFilter};

    export_service!();
    __export_service()
}

// Method used to save the candid interface to a file
#[test]
pub fn candid() {
    catalyze_shared::candid::save_candid_file("../../candid/index_group.did", __export_did_tmp_());
}
