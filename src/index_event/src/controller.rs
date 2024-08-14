use std::collections::HashMap;

use candid::Principal;
use catalyze_shared::{
    attendee::{Attendee, AttendeeEntry},
    event_collection::{EventCollection, EventCollectionEntry},
    event_with_attendees::EventWithAttendees,
    CanisterResult,
};
use common::{IDIter, IndexConfig, IndexConfigWithKeyIter, IndexController, ShardStorage};

use crate::{
    aliases::{Entry, EntryFilter, EntrySort, Key, Value},
    config::Config,
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
    pub fn attendees(&self) -> impl ShardStorage<Principal, Attendee> {
        self.config.attendees()
    }

    pub fn group_events(&self) -> impl ShardStorage<u64, EventCollection> {
        self.config.group_events()
    }

    pub async fn add_event(&self, value: Value) -> CanisterResult<(Key, Value)> {
        let key = self.config.key_iter().next()?;
        let resp = self.insert(key, value.clone()).await?;
        self.handle_new_event(key, value)?;
        Ok(resp)
    }

    pub async fn remove_event(&self, key: Key) -> CanisterResult<bool> {
        let (key, value) = self.get(key).await?;
        self.handle_remove_event(key, value)?;
        self.remove(key).await
    }

    pub async fn remove_many_events(&self, keys: Vec<Key>) -> CanisterResult<()> {
        let events = self.get_many(keys.clone()).await?;

        events.iter().try_for_each(|(key, value)| {
            self.handle_remove_event(*key, value.clone())?;
            Ok(())
        })?;

        controller().remove_many(keys).await
    }

    pub async fn update_event(&self, key: Key, new: Value) -> CanisterResult<(Key, Value)> {
        let (_, current) = self.get(key).await?;
        let resp = self.update(key, new.clone()).await?;
        self.handle_update_event(key, current, new.clone())?;
        Ok(resp)
    }

    pub async fn update_many_events(&self, list: Vec<Entry>) -> CanisterResult<Vec<Entry>> {
        let currents = self
            .get_many(list.iter().map(|(key, _)| *key).collect())
            .await?;

        let resp = self.update_many(list.clone()).await?;

        for (key, new) in list {
            let (_, current) = currents
                .iter()
                .find(|(k, _)| k == &key)
                .expect("Event not found");

            self.handle_update_event(key, current.clone(), new)?;
        }

        Ok(resp)
    }

    pub fn handle_new_event(&self, key: Key, value: EventWithAttendees) -> CanisterResult<()> {
        let group_id = value.group_id.expect("Group ID not set"); // TODO: FIX DIS

        value.attendees.members.iter().try_for_each(|(id, _)| {
            let (_, mut attendee) = self.get_attendee(*id);
            attendee.add_joined(key, group_id);
            self.config.attendees().upsert(*id, attendee)?;
            Ok(())
        })?;

        let (_, mut group_events) = self.get_group_events(group_id);
        group_events.add_event(key);
        self.config.group_events().upsert(group_id, group_events)?;

        Ok(())
    }

    fn handle_remove_event(&self, key: Key, value: Value) -> CanisterResult<()> {
        value.attendees.members.iter().try_for_each(|(id, _)| {
            let (_, mut attendee) = self.get_attendee(*id);
            attendee.remove_joined(key);
            self.config.attendees().upsert(*id, attendee)?;
            Ok(())
        })?;

        value.attendees.invites.iter().try_for_each(|(id, _)| {
            let (_, mut attendee) = self.get_attendee(*id);
            attendee.remove_invite(key);
            self.config.attendees().upsert(*id, attendee)?;
            Ok(())
        })?;

        let group_id = value.group_id.expect("Group ID not set"); // TODO: FIX DIS
        let (_, mut group_events) = self.get_group_events(group_id);
        group_events.remove_event(&key);
        self.config.group_events().upsert(group_id, group_events)?;

        Ok(())
    }

    fn handle_update_event(&self, key: Key, current: Value, new: Value) -> CanisterResult<()> {
        let current_invited = &current.attendees.invites;
        let current_members = &current.attendees.members;

        let new_invited = &new.attendees.invites;
        let new_members = &new.attendees.members;

        let invited_to_add = new_invited
            .iter()
            .filter(|(id, _)| !current_invited.contains_key(id))
            .collect::<HashMap<_, _>>();
        let invited_to_remove = current_invited
            .iter()
            .filter(|(id, _)| !new_invited.contains_key(id))
            .collect::<HashMap<_, _>>();

        let members_to_add = new_members
            .iter()
            .filter(|(id, _)| !current_members.contains_key(id))
            .collect::<HashMap<_, _>>();
        let members_to_remove = current_members
            .iter()
            .filter(|(id, _)| !new_members.contains_key(id))
            .collect::<HashMap<_, _>>();

        let group_id = new.group_id.expect("Group ID not set"); // TODO: FIX DIS

        for (id, invite) in invited_to_add {
            let (_, mut attendee) = self.get_attendee(*id);

            if attendee.is_event_invited(&key) {
                continue;
            }

            attendee.add_invite(
                key,
                group_id,
                invite.invite_type.clone(),
                invite.notification_id,
            );

            self.config.attendees().upsert(*id, attendee)?;
        }

        for (id, _) in invited_to_remove {
            let (_, mut attendee) = self.get_attendee(*id);

            if !attendee.is_event_invited(&key) {
                continue;
            }

            attendee.remove_invite(key);
            self.config.attendees().upsert(*id, attendee)?;
        }

        for (id, _) in members_to_add {
            let (_, mut attendee) = self.get_attendee(*id);

            if attendee.is_event_joined(&key) {
                continue;
            }

            attendee.add_joined(key, group_id);
            self.config.attendees().upsert(*id, attendee)?;
        }

        for (id, _) in members_to_remove {
            let (_, mut attendee) = self.get_attendee(*id);

            if !attendee.is_event_joined(&key) {
                continue;
            }

            attendee.remove_joined(key);
            self.config.attendees().upsert(*id, attendee)?;
        }

        Ok(())
    }

    fn get_attendee(&self, id: Principal) -> AttendeeEntry {
        self.config
            .attendees()
            .get_opt(id)
            .unwrap_or_else(|| (id, Default::default()))
    }

    fn get_group_events(&self, group_id: u64) -> EventCollectionEntry {
        self.config
            .group_events()
            .get_opt(group_id)
            .unwrap_or_else(|| (group_id, Default::default()))
    }
}

pub fn controller() -> Controller {
    Controller::default()
}
