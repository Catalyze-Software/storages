use catalyze_shared::{api_error::ApiError, CanisterResult, Filter, Sorter};
use ic_stable_structures::Storable;

use crate::{
    IDIter, IndexConfig, IndexController, IndexControllerStateful, Registry, ShardStorage,
};

pub async fn insert_by_key<V, F, S>(
    controller: impl IndexController<u64, V, F, S>,
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
    F: 'static + Filter<u64, V> + candid::CandidType + Clone + Send + Sync,
    S: 'static + Sorter<u64, V> + Default + candid::CandidType + Clone + Send + Sync,
{
    if controller.config().registry().exists(key) {
        return Err(duplicate_error(key));
    }

    controller.insert(handle_key(iterator, key)?, value).await
}

pub fn insert_by_key_stateful<V, F, S>(
    controller: impl IndexControllerStateful<u64, V, F, S>,
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
    F: 'static + Filter<u64, V> + candid::CandidType + Clone + Send + Sync,
    S: 'static + Sorter<u64, V> + Default + candid::CandidType + Clone + Send + Sync,
{
    if controller.storage().exists(key) {
        return Err(duplicate_error(key));
    }

    controller.insert(handle_key(iterator, key)?, value)
}

fn handle_key(iterator: impl IDIter, mut key: u64) -> CanisterResult<u64> {
    let current_key = iterator.get()?;

    // During the migration process, we need to ensure that the keys are inserted in order.
    // If the key is greater or equals to the current key, we need to update the iterator, thus
    // ensuring that the next key is greater than the current key.
    if key >= current_key {
        iterator.set(key)?;
        key = iterator.next()?;
    }

    Ok(key)
}

fn duplicate_error(key: u64) -> ApiError {
    ApiError::duplicate()
        .add_message("Key already exists")
        .add_info(key.to_string().as_str())
}
