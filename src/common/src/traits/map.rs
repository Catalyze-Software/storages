use catalyze_shared::{api_error::ApiError, CanisterResult, StaticStorageRef};
use ic_stable_structures::Storable;

pub trait MapStorage<K, V>
where
    K: 'static + Storable + Ord + Clone,
    V: 'static + Storable + Clone,
{
    fn name(&self) -> String;
    fn raw(&self) -> StaticStorageRef<K, V>;

    /// Insert a single entity by key
    /// # Arguments
    /// * `key` - The entity as key of the entity to insert
    /// * `value` - The entity to insert
    /// # Returns
    /// * `Result<(K, V), ApiError>` - The inserted entity if successful, otherwise an error
    /// # Note
    /// Does check if a entity with the same key already exists, if so returns an error
    fn insert_by_key(&self, key: K, value: V) -> CanisterResult<(K, V)> {
        self.raw().with(|data| {
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

    /// Get the total number of entries
    /// # Returns
    /// * `u64` - The total number of entries
    fn size(&self) -> u64 {
        self.raw().with(|data| data.borrow().len())
    }

    /// Get a single entity by key
    /// # Arguments
    /// * `key` - The key of the entity to get
    /// # Returns
    /// * `Result<(K, V), ApiError>` - The entity if found, otherwise an error
    fn get(&self, key: K) -> CanisterResult<(K, V)> {
        self.raw().with(|data| {
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

    /// Get multiple entities by key
    /// # Arguments
    /// * `keys` - The keys of the entities to get
    /// # Returns
    /// * `Vec<(K, V)>` - The entities if found, otherwise an empty vector
    fn get_many(&self, keys: Vec<K>) -> Vec<(K, V)> {
        self.raw().with(|data| {
            let mut entities = Vec::new();
            for key in keys {
                if let Some(value) = data.borrow().get(&key) {
                    entities.push((key, value));
                }
            }
            entities
        })
    }

    /// Get all entities by key
    /// # Returns
    /// * `Vec<(K, V)>` - The entities if found, otherwise an empty vector
    fn get_all(&self) -> Vec<(K, V)> {
        self.raw().with(|data| data.borrow().iter().collect())
    }

    /// Find a single entity by filter
    /// # Arguments
    /// * `filter` - The filter to apply
    /// # Returns
    /// * `Option<(K, V)>` - The entity if found, otherwise None
    fn find<F>(&self, filter: F) -> Option<(K, V)>
    where
        F: Fn(&K, &V) -> bool,
    {
        self.raw()
            .with(|data| data.borrow().iter().find(|(id, value)| filter(id, value)))
    }

    /// Find all entities by filter
    /// # Arguments
    /// * `filter` - The filter to apply
    /// # Returns
    /// * `Vec<(K, V)>` - The entities if found, otherwise an empty vector
    fn filter<F>(&self, filter: F) -> Vec<(K, V)>
    where
        F: Fn(&K, &V) -> bool,
    {
        self.raw().with(|data| {
            data.borrow()
                .iter()
                .filter(|(id, value)| filter(id, value))
                .collect()
        })
    }

    /// Update a single entity by key
    /// # Arguments
    /// * `key` - The key of the entity to update
    /// * `value` - The entity to update
    /// # Returns
    /// * `Result<(K, V), ApiError>` - The updated entity if successful, otherwise an error
    /// # Note
    /// Does check if a entity with the same key already exists, if not returns an error
    fn update(&self, key: K, value: V) -> CanisterResult<(K, V)> {
        self.raw().with(|data| {
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

    /// Remove a single entity by key
    /// # Arguments
    /// * `key` - The key of the entity to remove
    /// # Returns
    /// * `bool` - True if the entity was removed, otherwise false
    fn remove(&self, key: K) -> bool {
        self.raw()
            .with(|data| data.borrow_mut().remove(&key).is_some())
    }

    /// Remove a entities by keys
    /// # Arguments
    /// * `keys` - The keys of the entities to remove
    fn remove_many(&self, keys: Vec<K>) {
        self.raw().with(|data| {
            for key in keys {
                data.borrow_mut().remove(&key);
            }
        })
    }
}
