use std::time::Duration;

use candid::Principal;
use catalyze_shared::{
    referral::{Referral, REFERRAL_EXPIRATION},
    CanisterResult,
};
use common::{IndexConfig, IndexController};
use ic_cdk::api::time;
use ic_cdk_timers::{clear_timer, set_timer, TimerId};

use crate::{
    aliases::{Entry, EntryFilter, EntrySort, Key, Value},
    config::Config,
    state::REFERRAL_TIMERS,
};

#[derive(Default)]
pub struct Controller {
    config: Config,
}

impl IndexController<Key, Value, EntryFilter, EntrySort> for Controller {
    fn config(&self) -> impl IndexConfig<Key> {
        self.config.clone()
    }
}

impl Controller {
    pub async fn update_profile(&self, key: Key, value: Value) -> CanisterResult<Entry> {
        let (_, current) = self.get(key).await?;
        let (_, updated) = self.update(key, value).await?;

        // Remove new referrals after the expiration
        updated
            .references
            .referrals
            .clone()
            .into_iter()
            .filter(|(referral, _)| !current.is_referral_exists(*referral))
            .for_each(|(referral, data)| self.remove_referral_with_timer(key, referral, data));

        // Update existing timers if referrals was updated
        updated
            .references
            .referrals
            .clone()
            .into_iter()
            .filter(|(referral, data)| {
                if let Some(existing) = current.references.referrals.get(referral) {
                    existing.created_at != data.created_at
                } else {
                    false
                }
            })
            .for_each(|(referral, data)| {
                self.remove_timer(referral);
                self.remove_referral_with_timer(key, referral, data);
            });

        Ok((key, updated))
    }

    pub async fn init_timers(&self) {
        if let Ok(profiles) = self.get_all().await {
            profiles.into_iter().for_each(|entry| {
                self.remove_referrals(entry);
            });
        }
    }

    fn remove_referrals(&self, (key, profile): Entry) {
        profile
            .references
            .referrals
            .into_iter()
            .for_each(|(referral, data)| self.remove_referral_with_timer(key, referral, data));
    }

    fn remove_referral_with_timer(&self, profile_id: Key, referral: Principal, data: Referral) {
        if data.created_at + REFERRAL_EXPIRATION < time() {
            // Use spawn to keep sync interface
            self.spawn_referral_remove(profile_id, referral);
            return;
        }

        let delay = Duration::from_nanos(time() - data.created_at + REFERRAL_EXPIRATION);

        let timer_id = set_timer(delay, move || {
            controller().spawn_referral_remove(profile_id, referral);
        });

        self.set_timer_id(referral, timer_id);
    }

    pub fn spawn_referral_remove(&self, profile_id: Key, referral: Principal) {
        ic_cdk::spawn(async move {
            let (_, mut profile) = controller().get(profile_id).await.unwrap();
            profile.remove_referral(referral);
            let _ = controller().update(profile_id, profile).await;

            controller().remove_timer(referral);
        });
    }

    fn set_timer_id(&self, referral: Principal, timer_id: TimerId) {
        REFERRAL_TIMERS.with(|t| {
            t.borrow_mut().insert(referral, timer_id);
        });
    }

    pub fn remove_timer(&self, referral: Principal) {
        REFERRAL_TIMERS.with(|t| {
            if let Some(timer_id) = t.borrow().get(&referral).cloned() {
                clear_timer(timer_id);
                t.borrow_mut().remove(&referral);
            }
        });
    }
}

pub fn controller() -> Controller {
    Controller::default()
}
