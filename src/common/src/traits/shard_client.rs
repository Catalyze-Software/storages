use candid::Principal;
use catalyze_shared::{ic_call::ic_call, CanisterResult};

pub trait ShardClient<K, V, F>: Send + Sync
where
    K: candid::CandidType + for<'a> candid::Deserialize<'a>,
    V: candid::CandidType + for<'a> candid::Deserialize<'a>,
    F: candid::CandidType + Clone,
{
    fn get(
        &self,
        shard: Principal,
        id: K,
    ) -> impl std::future::Future<Output = CanisterResult<(K, V)>> + Sync + Send {
        ic_call(shard, "get", (id,))
    }

    fn get_many(
        &self,
        shard: Principal,
        keys: Vec<K>,
    ) -> impl std::future::Future<Output = CanisterResult<Vec<(K, V)>>> + Sync + Send {
        ic_call(shard, "get_many", (keys,))
    }

    fn get_all(
        &self,
        shard: Principal,
    ) -> impl std::future::Future<Output = CanisterResult<Vec<(K, V)>>> + Sync + Send {
        ic_call(shard, "get_all", ())
    }

    fn find(
        &self,
        shard: Principal,
        filters: Vec<F>,
    ) -> impl std::future::Future<Output = CanisterResult<Option<(K, V)>>> + Sync + Send {
        ic_call(shard, "find", (filters,))
    }

    fn filter(
        &self,
        shard: Principal,
        filters: Vec<F>,
    ) -> impl std::future::Future<Output = CanisterResult<Vec<(K, V)>>> + Sync + Send {
        ic_call(shard, "filter", (filters,))
    }

    fn insert(
        &self,
        shard: Principal,
        key: K,
        value: V,
    ) -> impl std::future::Future<Output = CanisterResult<(K, V)>> + Sync + Send {
        ic_call(shard, "insert", (key, value))
    }

    fn update(
        &self,
        shard: Principal,
        key: K,
        value: V,
    ) -> impl std::future::Future<Output = CanisterResult<(K, V)>> + Sync + Send {
        ic_call(shard, "update", (key, value))
    }

    fn remove(
        &self,
        shard: Principal,
        key: K,
    ) -> impl std::future::Future<Output = CanisterResult<bool>> + Sync + Send {
        ic_call(shard, "remove", (key,))
    }

    fn remove_many(
        &self,
        shard: Principal,
        keys: Vec<K>,
    ) -> impl std::future::Future<Output = CanisterResult<()>> + Sync + Send {
        ic_call(shard, "remove_many", (keys,))
    }
}
