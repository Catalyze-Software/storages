use ic_cdk::query;

#[ic_cdk::query]
fn greet(name: String) -> String {
    common::greet(name)
}

#[query]
fn icts_name() -> String {
    env!("CARGO_PKG_NAME").to_string()
}

#[query]
fn icts_version() -> String {
    env!("CARGO_PKG_VERSION").to_string()
}

// Hacky way to expose the candid interface to the outside world
#[query(name = "__get_candid_interface_tmp_hack")]
pub fn __export_did_tmp_() -> String {
    use candid::export_service;
    export_service!();
    __export_service()
}

// Method used to save the candid interface to a file
#[test]
pub fn candid() {
    catalyze_shared::candid::save_candid_file(
        "../../candid/shard_profile.did",
        __export_did_tmp_(),
    );
}
