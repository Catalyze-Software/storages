use std::{collections::HashMap, fmt::Debug};

use candid::{CandidType, Principal};
use catalyze_shared::{
    event_collection::EventCollection,
    event_with_attendees::EventWithAttendees,
    general_structs::members::Members,
    group_with_members::GroupWithMembers,
    member::{Invite, Join},
    member_collection::MemberCollection,
    profile_with_refs::ProfileWithRefs,
};
use serde::{de::DeserializeOwned, Serialize};

use crate::{
    canister_methods::Canister, event_mapper::EventMapArgs, group_mapper::GroupMapArgs,
    index_methods::IndexCalls, profile_mapper::ProfileMapArgs, proxy_methods::ProxyCalls,
    utils::context,
};

pub struct Migrate;

impl Migrate {
    pub async fn profiles() -> eyre::Result<(u32, u32)> {
        let proxy_profiles = ProxyCalls::get_profiles().await?;
        let proxy_members = ProxyCalls::get_members().await?;
        let proxy_attendees = ProxyCalls::get_attendees().await?;
        let proxy_user_notifications = ProxyCalls::get_user_notifications().await?;

        let mut mapped: HashMap<Principal, ProfileWithRefs> = HashMap::new();

        for (principal, profile) in proxy_profiles {
            if principal.as_slice().len() < 29 {
                continue;
            }
            let member = proxy_members
                .clone()
                .into_iter()
                .find(|(key, _)| key == &principal)
                .map(|(_, member)| member)
                .unwrap_or_default();

            let mut groups = member.joined.keys().cloned().collect::<Vec<u64>>();
            groups.append(&mut member.invites.keys().cloned().collect());

            let attendee = proxy_attendees
                .clone()
                .into_iter()
                .find(|(key, _)| key == &principal)
                .map(|(_, attendee)| attendee)
                .unwrap_or_default();

            let mut events = attendee.joined.keys().cloned().collect::<Vec<u64>>();

            events.append(&mut attendee.invites.keys().cloned().collect());

            let user_notifications = proxy_user_notifications
                .clone()
                .into_iter()
                .find(|(key, _)| key == &principal)
                .map(|(_, notification)| notification)
                .unwrap_or_default();

            mapped.insert(
                principal,
                ProfileMapArgs {
                    profile: profile.clone(),
                    groups,
                    events,
                    user_notifications,
                }
                .into(),
            );
        }

        migrate::<Principal, ProfileWithRefs>(
            "profiles",
            mapped.into_iter().collect(),
            Some(context().indexes.profiles),
        )
        .await
    }

    pub async fn groups() -> eyre::Result<(u32, u32)> {
        let proxy_groups = ProxyCalls::get_groups().await?;
        let proxy_events = ProxyCalls::get_group_events().await?;
        let proxy_group_members = ProxyCalls::get_group_members().await?;
        let proxy_members = ProxyCalls::get_members().await?;

        let mut mapped: HashMap<u64, GroupWithMembers> = HashMap::new();

        for (id, group) in proxy_groups {
            let events = proxy_events
                .clone()
                .into_iter()
                .find(|(key, _)| key == &id)
                .unwrap_or((id, EventCollection::new()));

            let group_members = proxy_group_members
                .clone()
                .into_iter()
                .find(|(key, _)| key == &id)
                .unwrap_or((id, MemberCollection::new()));

            let mut mapped_members: HashMap<Principal, Join> = HashMap::new();

            for member_principal in group_members.1.get_member_principals() {
                let member = proxy_members
                    .clone()
                    .into_iter()
                    .find(|(key, _)| key == &member_principal);

                if let Some((_, member)) = member {
                    let joined = member.get_joined(&id);
                    if let Some(joined) = joined {
                        mapped_members.insert(
                            member_principal,
                            Join {
                                roles: joined.roles,
                                updated_at: joined.updated_at,
                                created_at: joined.created_at,
                            },
                        );
                    }
                }
            }

            let mut mapped_invites: HashMap<Principal, Invite> = HashMap::new();

            for invite_principal in group_members.1.get_invite_principals() {
                let invitee = proxy_members
                    .clone()
                    .into_iter()
                    .find(|(key, _)| key == &invite_principal);

                if let Some((_, invitee)) = invitee {
                    let invite = invitee.get_invite(&id);
                    if let Some(invite) = invite {
                        mapped_invites.insert(
                            invite_principal,
                            Invite {
                                invite_type: invite.invite_type,
                                notification_id: invite.notification_id,
                                updated_at: invite.updated_at,
                                created_at: invite.created_at,
                            },
                        );
                    }
                }
            }

            mapped.insert(
                id,
                GroupMapArgs {
                    group: group.clone(),
                    events: events.1.get_event_ids(),
                    members: Members {
                        members: mapped_members,
                        invites: mapped_invites,
                        special_members: group.special_members,
                        roles: group.roles,
                    },
                }
                .into(),
            );
        }

        migrate(
            "groups",
            mapped.into_iter().collect(),
            Some(context().indexes.groups),
        )
        .await
    }

    pub async fn events() -> eyre::Result<(u32, u32)> {
        let proxy_events = ProxyCalls::get_events().await?;
        let proxy_attendees = ProxyCalls::get_attendees().await?;
        let proxy_event_attendees = ProxyCalls::get_event_attendees().await?;

        let mut mapped: HashMap<u64, EventWithAttendees> = HashMap::new();

        for (id, event) in proxy_events {
            let event_attendees = proxy_event_attendees
                .clone()
                .into_iter()
                .find(|(key, _)| key == &id)
                .unwrap_or((id, MemberCollection::new()));

            let mut mapped_attendees: HashMap<Principal, Join> = HashMap::new();

            for attendee_principal in event_attendees.1.get_member_principals() {
                let attendee = proxy_attendees
                    .clone()
                    .into_iter()
                    .find(|(key, _)| key == &attendee_principal);

                if let Some((_, member)) = attendee {
                    let joined = member.get_joined(&id);
                    if let Some(joined) = joined {
                        mapped_attendees.insert(
                            attendee_principal,
                            Join {
                                roles: vec![],
                                updated_at: joined.updated_at,
                                created_at: joined.created_at,
                            },
                        );
                    }
                }
            }

            let mut mapped_invites: HashMap<Principal, Invite> = HashMap::new();

            for invite_principal in event_attendees.1.get_invite_principals() {
                let invitee = proxy_attendees
                    .clone()
                    .into_iter()
                    .find(|(key, _)| key == &invite_principal);

                if let Some((_, invitee)) = invitee {
                    let invite = invitee.get_invite(&id);
                    if let Some(invite) = invite {
                        mapped_invites.insert(
                            invite_principal,
                            Invite {
                                invite_type: invite.invite_type,
                                notification_id: invite.notification_id,
                                updated_at: invite.updated_at,
                                created_at: invite.created_at,
                            },
                        );
                    }
                }
            }

            mapped.insert(
                id,
                EventMapArgs {
                    event,
                    attendees: Members {
                        members: mapped_attendees,
                        invites: mapped_invites,
                        special_members: HashMap::new(),
                        roles: vec![],
                    },
                }
                .into(),
            );
        }

        migrate(
            "events",
            mapped.into_iter().collect(),
            Some(context().indexes.events),
        )
        .await
    }

    pub async fn reports() -> eyre::Result<(u32, u32)> {
        let proxy_reports = ProxyCalls::get_reports().await?;
        migrate("reports", proxy_reports, Some(context().indexes.reports)).await
    }

    pub async fn friend_requests() -> eyre::Result<(u32, u32)> {
        let friend_requests = ProxyCalls::get_friend_requests().await?;
        migrate(
            "friend_requests",
            friend_requests,
            Some(context().indexes.friend_requests),
        )
        .await
    }

    pub async fn notifications() -> eyre::Result<(u32, u32)> {
        let notifications = ProxyCalls::get_notifications().await?;
        migrate(
            "notifications",
            notifications,
            Some(context().indexes.notifications),
        )
        .await
    }

    pub async fn boosts() -> eyre::Result<(u32, u32)> {
        let proxy_boosts = ProxyCalls::get_boosted().await?;

        migrate("boosts", proxy_boosts, Some(context().indexes.boosted)).await
    }

    pub async fn all() -> eyre::Result<()> {
        // let profiles = Migrate::profiles().await?;
        // let groups = Migrate::groups().await?;
        let events = Migrate::events().await?;
        let reports = Migrate::reports().await?;
        let friend_requests = Migrate::friend_requests().await?;
        let notifications = Migrate::notifications().await?;
        let boosted = Migrate::boosts().await?;

        println!(
            "Migration completed: events: {:?}, reports: {:?}, friend_requests: {:?}, notifications: {:?}, boosted: {:?}",
             events, reports, friend_requests, notifications, boosted
        );

        Ok(())
    }
}

pub async fn migrate<K, V>(
    reference: &str,
    data: Vec<(K, V)>,
    index: Option<Canister>,
) -> eyre::Result<(u32, u32)>
where
    K: CandidType + Clone + DeserializeOwned + Serialize + Debug,
    V: CandidType + Clone + DeserializeOwned + Serialize + Debug,
{
    let mut success: u32 = 0;
    let mut failed: u32 = 0;

    // If the index is not provided, we don't need to migrate the data
    if let Some(index) = index {
        println!("Migrating {} {}", data.len(), reference);

        let index = IndexCalls::new(index);
        for (key, value) in data {
            match index.insert(key.clone(), value.clone()).await {
                Ok(_) => {
                    println!("Item {:?} migrated successfully", key);
                    success += 1;
                }
                Err(e) => {
                    println!("{:?}", value);
                    // println!("Failed to migrate item {:?}: {:?}", key, e);
                    failed += 1;
                }
            }
        }
    } else {
        println!("Skipping migration of {} {}", data.len(), reference);
    }
    Ok((success, failed))
}
