use catalyze_shared::{api_error::ApiError, CanisterResult};
use ic_stable_structures::Storable;

use crate::{IDIter, IDMap, IndexController};

pub async fn insert_by_key<V, F>(
    controller: impl IndexController<u64, V, F>,
    iterator: impl IDIter,
    key: u64,
    value: V,
) -> CanisterResult<(u64, V)>
where
    V: 'static
        + candid::CandidType
        + for<'a> candid::Deserialize<'a>
        + Storable
        + Clone
        + Send
        + Sync,
    F: 'static + candid::CandidType + Clone + Send + Sync,
{
    if controller.ids().exists(key) {
        return Err(ApiError::unexpected()
            .add_message("Key already exists")
            .add_info(key.to_string().as_str()));
    }

    iterator.set(key)?;

    let key = iterator.next()?;
    let entry = controller.insert(key, value).await?;

    Ok(entry)
}
