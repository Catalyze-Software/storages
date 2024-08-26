use catalyze_shared::{api_error::ApiError, CellStorage};
use ic_cdk::caller;

use crate::state;

pub fn is_proxy() -> Result<(), String> {
    let storage = state::proxies().get().map_err(|e| e.to_string())?;

    if !storage.to_vec().contains(&caller()) {
        return Err(ApiError::unauthorized().to_string());
    }

    Ok(())
}
