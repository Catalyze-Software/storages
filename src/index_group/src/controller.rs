use std::collections::HashMap;

use candid::Principal;
use catalyze_shared::{
    old_member::{Member, MemberEntry},
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
    pub fn members(&self) -> impl ShardStorage<Principal, Member> {
        self.config.members()
    }

    pub async fn add_group(&self, value: Value) -> CanisterResult<(Key, Value)> {
        let key = self.config.key_iter().next()?;
        let resp = self.insert(key, value.clone()).await?;
        self.handle_new_group(key, value)?;
        Ok(resp)
    }

    pub async fn remove_group(&self, key: Key) -> CanisterResult<bool> {
        let (key, value) = self.get(key).await?;
        self.handle_remove_group(key, value)?;
        self.remove(key).await
    }

    pub async fn remove_many_groups(&self, keys: Vec<Key>) -> CanisterResult<()> {
        let groups = self.get_many(keys.clone()).await?;
        controller().remove_many(keys).await?;

        groups.iter().try_for_each(|(key, value)| {
            self.handle_remove_group(*key, value.clone())?;
            Ok(())
        })?;

        Ok(())
    }

    pub async fn update_group(&self, key: Key, new: Value) -> CanisterResult<(Key, Value)> {
        let (_, current) = self.get(key).await?;
        let resp = self.update(key, new.clone()).await?;
        self.handle_update_group(key, current, new.clone())?;
        Ok(resp)
    }

    pub async fn update_many_groups(&self, list: Vec<Entry>) -> CanisterResult<Vec<Entry>> {
        let currents = self
            .get_many(list.iter().map(|(key, _)| *key).collect())
            .await?;

        let resp = self.update_many(list.clone()).await?;

        for (key, new) in list {
            let (_, current) = currents
                .iter()
                .find(|(k, _)| k == &key)
                .expect("Group not found");

            self.handle_update_group(key, current.clone(), new)?;
        }

        Ok(resp)
    }

    pub fn handle_new_group(&self, key: Key, value: Value) -> CanisterResult<()> {
        value.members.members.iter().try_for_each(|(id, join)| {
            let (_, mut member) = self.get_member(*id);
            member.add_joined(key, join.roles.clone());
            self.config.members().upsert(*id, member)?;
            Ok(())
        })?;

        Ok(())
    }

    fn handle_remove_group(&self, key: Key, value: Value) -> CanisterResult<()> {
        value.members.members.iter().try_for_each(|(id, _)| {
            let (_, mut member) = self.get_member(*id);
            member.remove_joined(key);
            self.config.members().upsert(*id, member)?;
            Ok(())
        })?;

        value.members.invites.iter().try_for_each(|(id, _)| {
            let (_, mut member) = self.get_member(*id);
            member.remove_invite(key);
            self.config.members().upsert(*id, member)?;
            Ok(())
        })?;

        Ok(())
    }

    fn handle_update_group(&self, key: Key, current: Value, new: Value) -> CanisterResult<()> {
        let current_invited = &current.members.invites;
        let current_members = &current.members.members;

        let new_invited = &new.members.invites;
        let new_members = &new.members.members;

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

        let members_to_update = current_members
            .iter()
            .filter(|(id, _)| new_members.contains_key(id))
            .collect::<HashMap<_, _>>();

        for (id, invite) in invited_to_add {
            let (_, mut member) = self.get_member(*id);

            if member.is_group_invited(&key) {
                continue;
            }

            member.add_invite(key, invite.invite_type.clone(), invite.notification_id);

            self.config.members().upsert(*id, member)?;
        }

        for (id, _) in invited_to_remove {
            let (_, mut member) = self.get_member(*id);

            if !member.is_group_invited(&key) {
                continue;
            }

            member.remove_invite(key);
            self.config.members().upsert(*id, member)?;
        }

        for (id, join) in members_to_add {
            let (_, mut member) = self.get_member(*id);

            if member.is_group_joined(&key) {
                continue;
            }

            member.add_joined(key, join.roles.clone());
            self.config.members().upsert(*id, member)?;
        }

        for (id, _) in members_to_remove {
            let (_, mut member) = self.get_member(*id);

            if !member.is_group_joined(&key) {
                continue;
            }

            member.remove_joined(key);
            self.config.members().upsert(*id, member)?;
        }

        for (id, join) in members_to_update {
            let (_, mut member) = self.get_member(*id);

            if !member.is_group_joined(&key) {
                continue;
            }

            member.replace_roles(&key, join.roles.clone());
            self.config.members().upsert(*id, member)?;
        }

        Ok(())
    }

    fn get_member(&self, id: Principal) -> MemberEntry {
        self.config
            .members()
            .get_opt(id)
            .unwrap_or_else(|| (id, Default::default()))
    }
}

pub fn controller() -> Controller {
    Controller::default()
}
