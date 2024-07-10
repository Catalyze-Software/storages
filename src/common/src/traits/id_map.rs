use candid::Principal;
use catalyze_shared::{api_error::ApiError, CanisterResult, StaticStorageRef};
use ic_stable_structures::Storable;

pub trait IDMap<K: 'static + Storable + Ord + Clone>: Send + Sync {
    fn name(&self) -> String;
    fn raw(&self) -> StaticStorageRef<K, Principal>;

    fn shard_by_id(&self, id: K) -> CanisterResult<Principal> {
        self.raw().with(|data| {
            data.borrow().get(&id).ok_or(
                ApiError::not_found()
                    .add_method_name("get")
                    .add_info(self.name().as_str()),
            )
        })
    }

    fn insert(&self, id: K, principal: Principal) -> CanisterResult<(K, Principal)> {
        self.raw().with(|data| {
            if data.borrow().contains_key(&id) {
                return Err(ApiError::duplicate()
                    .add_method_name("insert")
                    .add_info(self.name().as_str())
                    .add_message("Key already exists"));
            }

            data.borrow_mut().insert(id.clone(), principal);
            Ok((id, principal))
        })
    }

    fn exists(&self, id: K) -> bool {
        self.raw().with(|data| data.borrow().contains_key(&id))
    }
}
