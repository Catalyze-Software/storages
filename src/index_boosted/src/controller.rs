use std::{cell::RefCell, collections::HashMap, time::Duration};

use catalyze_shared::{CanisterResult, StaticStorageRef};
use common::{IDIter, IndexConfigWithKeyIter, IndexControllerStateful};
use ic_cdk::api::time;
use ic_cdk_timers::{clear_timer, set_timer, TimerId};

use crate::{
    aliases::{Entry, EntryFilter, EntrySort, Key, Value, DATA_KIND},
    config::config,
    state::DATA,
};

thread_local! {
    pub static TIMERS: RefCell<HashMap<u64, TimerId>> = RefCell::new(HashMap::default());
}

#[derive(Default)]
pub struct Controller;

impl IndexControllerStateful<Key, Value, EntryFilter, EntrySort> for Controller {
    fn name(&self) -> String {
        DATA_KIND.to_owned()
    }

    fn raw(&self) -> StaticStorageRef<Key, Value> {
        &DATA
    }
}

impl Controller {
    pub fn new_boost(&self, value: Value) -> CanisterResult<Entry> {
        let (key, value) = controller().insert(config().key_iter().next()?, value)?;
        self.set_timer(key, value.seconds);
        Ok((key, value))
    }

    pub fn new_boost_many(&self, list: Vec<Value>) -> CanisterResult<Vec<Entry>> {
        let list = list.into_iter().try_fold(vec![], |mut acc, value| {
            let key = config().key_iter().next()?;
            acc.push((key, value));
            Ok(acc)
        })?;

        let list = controller().insert_many(list)?;

        list.clone().into_iter().for_each(|(key, value)| {
            self.set_timer(key, value.seconds);
        });

        Ok(list)
    }

    pub fn remove_boost(&self, key: Key) -> CanisterResult<bool> {
        if let Some(timer_id) = self.get_timer_id(key) {
            clear_timer(timer_id);
        }
        self.remove_timer_id(key);
        self.remove(key)
    }

    pub fn remove_boost_many(&self, keys: Vec<Key>) -> CanisterResult<()> {
        keys.into_iter().try_for_each(|key| {
            if let Some(timer_id) = self.get_timer_id(key) {
                clear_timer(timer_id);
            }
            self.remove_timer_id(key);
            self.remove(key)?;
            Ok(())
        })
    }

    pub fn update_boost(&self, key: Key, value: Value) -> CanisterResult<Entry> {
        // Get and clear the existing timer
        if let Some(timer_id) = self.get_timer_id(key) {
            clear_timer(timer_id);
        }

        let (key, value) = self.insert(key, value)?;

        // Remove the old timer and set a new timer with the updated seconds
        self.set_timer(key, value.seconds);

        Ok((key, value))
    }

    pub fn update_boost_many(&self, list: Vec<Entry>) -> CanisterResult<Vec<Entry>> {
        let list = list.into_iter().try_fold(vec![], |mut acc, (key, value)| {
            // Get and clear the existing timer
            if let Some(timer_id) = self.get_timer_id(key) {
                clear_timer(timer_id);
            }

            acc.push((key, value));
            Ok(acc)
        })?;

        let list = controller().update_many(list)?;

        list.clone().into_iter().for_each(|(key, value)| {
            self.set_timer(key, value.seconds);
        });

        Ok(list)
    }

    pub fn set_timer(&self, key: Key, seconds: u64) {
        let timer_id = set_timer(Duration::from_secs(seconds), move || {
            let _ = controller().remove_boost(key);
        });

        self.set_timer_id(key, timer_id);
    }

    pub fn start_timers_after_upgrade(&self) -> CanisterResult<()> {
        self.get_all()?.into_iter().for_each(|(key, value)| {
            let time_left: u64 = Duration::from_nanos(value.updated_at).as_secs() + value.seconds;
            let seconds = time_left - Duration::from_nanos(time()).as_secs();
            self.set_timer(key, seconds);
        });

        Ok(())
    }

    fn set_timer_id(&self, boost_id: u64, timer_id: TimerId) {
        TIMERS.with(|t| {
            t.borrow_mut().insert(boost_id, timer_id);
        });
    }

    fn get_timer_id(&self, boost_id: u64) -> Option<TimerId> {
        TIMERS.with(|t| t.borrow().get(&boost_id).cloned())
    }

    fn remove_timer_id(&self, boost_id: u64) {
        TIMERS.with(|t| {
            t.borrow_mut().remove(&boost_id);
        });
    }
}

pub fn controller() -> Controller {
    Controller
}
