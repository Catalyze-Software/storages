use candid::{utils::ArgumentEncoder, CandidType, Principal};
use catalyze_shared::{api_error::ApiError, CanisterResult};
use serde::Deserialize;

#[derive(CandidType, Deserialize)]
pub enum CanisterCallResult<T> {
    Ok(T),
    Err(ApiError),
}

impl<T> From<CanisterCallResult<T>> for CanisterResult<T> {
    fn from(val: CanisterCallResult<T>) -> Self {
        match val {
            CanisterCallResult::Ok(value) => Ok(value),
            CanisterCallResult::Err(err) => Err(err),
        }
    }
}

pub trait ShardClient<K, V>: Send + Sync
where
    K: candid::CandidType + for<'a> candid::Deserialize<'a>,
    V: candid::CandidType + for<'a> candid::Deserialize<'a>,
{
    fn call<A: ArgumentEncoder, R: candid::CandidType + for<'a> candid::Deserialize<'a>>(
        &self,
        shard: Principal,
        method: &str,
        args: A,
    ) -> impl std::future::Future<Output = CanisterResult<R>> + Sync + Send {
        let fut = ic_cdk::call::<A, (CanisterCallResult<R>,)>(shard, method, args);

        let method = method.to_string();

        async move {
            let (res,) = fut.await.map_err(|e| {
                ApiError::unexpected()
                    .add_message("Failed to call shard")
                    .add_info(format!("Method: {method}, error: {:?}", e).as_str())
            })?;

            res.into()
        }
    }

    fn get(
        &self,
        shard: Principal,
        id: K,
    ) -> impl std::future::Future<Output = CanisterResult<(K, V)>> + Sync + Send {
        self.call(shard, "get", (id,))
    }

    fn get_many(
        &self,
        shard: Principal,
        keys: Vec<K>,
    ) -> impl std::future::Future<Output = CanisterResult<Vec<(K, V)>>> + Sync + Send {
        self.call(shard, "get_many", (keys,))
    }

    fn get_all(
        &self,
        shard: Principal,
    ) -> impl std::future::Future<Output = CanisterResult<Vec<(K, V)>>> + Sync + Send {
        self.call(shard, "get_all", ())
    }

    fn insert(
        &self,
        shard: Principal,
        key: K,
        value: V,
    ) -> impl std::future::Future<Output = CanisterResult<(K, V)>> + Sync + Send {
        self.call(shard, "insert", (key, value))
    }

    fn update(
        &self,
        shard: Principal,
        key: K,
        value: V,
    ) -> impl std::future::Future<Output = CanisterResult<(K, V)>> + Sync + Send {
        self.call(shard, "update", (key, value))
    }

    fn remove(
        &self,
        shard: Principal,
        key: K,
    ) -> impl std::future::Future<Output = CanisterResult<bool>> + Sync + Send {
        self.call(shard, "remove", (key,))
    }

    fn remove_many(
        &self,
        shard: Principal,
        keys: Vec<K>,
    ) -> impl std::future::Future<Output = CanisterResult<()>> + Sync + Send {
        self.call(shard, "remove_many", (keys,))
    }
}
