use candid::Principal;
use catalyze_shared::api_error::ApiError;
use ic_cdk::caller;

use crate::Principals;

pub fn is_authorized() -> Result<(), String> {
    if caller() != Principal::anonymous() {
        return Err(ApiError::unauthorized()
            .add_message("Unauthorized, caller is not anonymous")
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
