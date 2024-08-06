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

    let mut key = key;
    let current_key = iterator.get()?;

    // During the migration process, we need to ensure that the keys are inserted in order.
    // If the key is greater or equals to the current key, we need to update the iterator, thus
    // ensuring that the next key is greater than the current key.
    if key >= current_key {
        iterator.set(key)?;
        key = iterator.next()?;
    }

    controller.insert(key, value).await
}
