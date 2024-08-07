use candid::Principal;
use catalyze_shared::api_error::ApiError;
use ic_cdk::caller;

use crate::Principals;

pub fn is_authorized() -> Result<(), String> {
    if caller() == Principal::anonymous() {
        return Err(ApiError::unauthorized()
            .add_message("Unauthorized, caller is anonymous")
            .to_string());
    }

    Ok(())
}

pub fn is_proxy(proxies: Principals) -> Result<(), String> {
    is_authorized()?;

    if proxies.to_vec().contains(&caller()) {
        return Ok(());
    }

    Err(ApiError::unauthorized()
        .add_message("Unauthorized, caller is not a proxy")
        .to_string())
}

pub fn is_storage_index(storage_index: Principal) -> Result<(), String> {
    is_authorized()?;

    if storage_index == caller() {
        return Ok(());
    }

    Err(ApiError::unauthorized()
        .add_message("Unauthorized, caller is not the storage index")
        .to_string())
}

pub fn is_developer() -> Result<(), String> {
    is_authorized()?;

    let developers = [
        "ledm3-52ncq-rffuv-6ed44-hg5uo-iicyu-pwkzj-syfva-heo4k-p7itq-aqe",
        // staging/develop
        "syzio-xu6ca-burmx-4afo2-ojpcw-e75j3-m67o5-s5bes-5vvsv-du3t4-wae",
    ];

    if !developers.contains(&caller().to_text().as_str()) {
        return Err(ApiError::unauthorized()
            .add_message("Unauthorized")
            .to_string());
    }

    Ok(())
}

pub fn is_migration() -> Result<(), String> {
    is_authorized()?;

    let migration_identity = "kgc2f-v7q43-4p5vn-kgtko-pg2y2-frrom-ynwmi-ywrvl-kdmlz-cdh25-cae";

    if migration_identity == caller().to_string().as_str() {
        return Ok(());
    }

    Err(ApiError::unauthorized()
        .add_message("Unauthorized, caller is not the specific principal")
        .to_string())
}
