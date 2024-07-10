use candid::Principal;
use catalyze_shared::CanisterResult;

use crate::Filter;

pub trait Shard<K, V>
where
    K: candid::CandidType,
    V: candid::CandidType,
{
    fn get(
        &self,
        shard: Principal,
        id: K,
    ) -> impl std::future::Future<Output = CanisterResult<(K, V)>> + Sync + Send;

    fn get_many(
        &self,
        shard: Principal,
        keys: Vec<K>,
    ) -> impl std::future::Future<Output = CanisterResult<Vec<(K, V)>>> + Sync + Send;

    fn get_all(
        &self,
        shard: Principal,
    ) -> impl std::future::Future<Output = CanisterResult<Vec<(K, V)>>> + Sync + Send;

    fn find(
        &self,
        shard: Principal,
        filters: Vec<impl Filter<V>>,
    ) -> impl std::future::Future<Output = CanisterResult<Option<(K, V)>>> + Sync + Send;

    fn filter(
        &self,
        shard: Principal,
        filters: Vec<impl Filter<V>>,
    ) -> impl std::future::Future<Output = CanisterResult<Vec<(K, V)>>> + Sync + Send;

    fn insert(
        &self,
        shard: Principal,
        key: K,
        value: V,
    ) -> impl std::future::Future<Output = CanisterResult<(K, V)>> + Sync + Send;

    fn update(
        &self,
        shard: Principal,
        key: K,
        value: V,
    ) -> impl std::future::Future<Output = CanisterResult<(K, V)>> + Sync + Send;

    fn remove(
        &self,
        shard: Principal,
        key: K,
    ) -> impl std::future::Future<Output = CanisterResult<bool>> + Sync + Send;

    fn remove_many(
        &self,
        shard: Principal,
        keys: Vec<K>,
    ) -> impl std::future::Future<Output = CanisterResult<()>> + Sync + Send;
}
