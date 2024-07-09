use catalyze_shared::api_error::ApiError;
use ic_cdk::caller;

use crate::Principals;

pub fn is_proxy(proxies: Principals) -> Result<(), String> {
    if proxies.to_vec().contains(&caller()) {
        return Ok(());
    }

    Err(ApiError::unauthorized()
        .add_message("Unauthorized, caller is not a proxy")
        .to_string())
}
