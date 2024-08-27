use catalyze_shared::CellStorage;
use common::is_proxy as _is_proxy;

use crate::state;

pub fn is_proxy() -> Result<(), String> {
    let storage = state::proxies().get().map_err(|e| e.to_string())?;
    _is_proxy(storage)
}
