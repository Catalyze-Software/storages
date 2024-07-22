use catalyze_shared::{api_error::ApiError, CanisterResult, StaticStorageRef};
use ic_stable_structures::Storable;

pub trait ShardStorage<K, V>
where
    K: 'static + Storable + Ord + Clone,
    V: 'static + Storable + Clone,
{
    fn name(&self) -> String;
    fn storage(&self) -> StaticStorageRef<K, V>;

    fn size(&self) -> u64 {
        self.storage().with(|data| data.borrow().len())
    }

    fn insert_by_key(&self, key: K, value: V) -> CanisterResult<(K, V)> {
        self.storage().with(|data| {
            if data.borrow().contains_key(&key) {
                return Err(ApiError::duplicate()
                    .add_method_name("insert_by_key")
                    .add_info(self.name().as_str())
                    .add_message("Key already exists"));
            }

            data.borrow_mut().insert(key.clone(), value.clone());
            Ok((key, value))
        })
    }

    fn get(&self, key: K) -> CanisterResult<(K, V)> {
        self.storage().with(|data| {
            data.borrow()
                .get(&key)
                .ok_or(
                    ApiError::not_found()
                        .add_method_name("get")
                        .add_info(self.name().as_str()),
                )
                .map(|value| (key, value))
        })
    }

    fn get_many(&self, keys: Vec<K>) -> Vec<(K, V)> {
        self.storage().with(|data| {
            let mut entities = Vec::new();
            for key in keys {
                if let Some(value) = data.borrow().get(&key) {
                    entities.push((key, value));
                }
            }
            entities
        })
    }

    fn get_all(&self) -> Vec<(K, V)> {
        self.storage().with(|data| data.borrow().iter().collect())
    }

    fn find<F>(&self, filter: F) -> Option<(K, V)>
    where
        F: Fn(&K, &V) -> bool,
    {
        self.storage()
            .with(|data| data.borrow().iter().find(|(id, value)| filter(id, value)))
    }

    fn filter<F>(&self, filter: F) -> Vec<(K, V)>
    where
        F: Fn(&K, &V) -> bool,
    {
        self.storage().with(|data| {
            data.borrow()
                .iter()
                .filter(|(id, value)| filter(id, value))
                .collect()
        })
    }

    fn update(&self, key: K, value: V) -> CanisterResult<(K, V)> {
        self.storage().with(|data| {
            if !data.borrow().contains_key(&key) {
                return Err(ApiError::not_found()
                    .add_method_name("update")
                    .add_info(self.name().as_str())
                    .add_message("Key does not exist"));
            }

            data.borrow_mut().insert(key.clone(), value.clone());
            Ok((key, value))
        })
    }

    fn remove(&self, key: K) -> bool {
        self.storage()
            .with(|data| data.borrow_mut().remove(&key).is_some())
    }

    fn remove_many(&self, keys: Vec<K>) {
        self.storage().with(|data| {
            for key in keys {
                data.borrow_mut().remove(&key);
            }
        })
    }
}
