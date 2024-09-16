use catalyze_shared::{
    general_structs::{
        members::Members, metadata::Metadata, privacy::Privacy, references::References,
    },
    group::Group,
    group_with_members::GroupWithMembers,
};

pub struct GroupMapArgs {
    pub group: Group,
    pub members: Members,
    pub events: Vec<u64>,
}

impl From<GroupMapArgs> for GroupWithMembers {
    fn from(args: GroupMapArgs) -> Self {
        let GroupMapArgs {
            group,
            members,
            events,
        } = args;
        GroupWithMembers {
            metadata: Metadata {
                name: group.name,
                description: group.description,
                banner_image: group.banner_image,
                image: group.image,
                location: group.location,
                website: group.website,
            },
            events,
            created_by: group.created_by,
            created_on: group.created_on,
            is_deleted: group.is_deleted,
            members,
            privacy: Privacy {
                privacy_type: group.privacy,
                privacy_gated_type_amount: group.privacy_gated_type_amount,
            },
            matrix_space_id: group.matrix_space_id,
            owner: group.owner,
            updated_on: group.updated_on,
            wallets: group.wallets,
            references: References {
                notification_id: group.notification_id,
                tags: group.tags,
            },
        }
    }
}

// impl GroupWithMembers {
//     async fn migrate_to_shards() -> eyre::Result<(u32, u32)> {
//         let proxy_groups = ProxyCalls::get_groups().await?;
//         let proxy_events = ProxyCalls::get_group_events().await?;
//         let proxy_group_members = ProxyCalls::get_group_members().await?;
//         let proxy_members = ProxyCalls::get_members().await?;

//         let mut mapped: Vec<(u64, GroupWithMembers)> = vec![];

//         for (id, group) in proxy_groups {
//             let events = proxy_events
//                 .clone()
//                 .into_iter()
//                 .find(|(key, _)| key == &id)
//                 .unwrap_or((id, EventCollection::new()));

//             let group_members = proxy_group_members
//                 .clone()
//                 .into_iter()
//                 .find(|(key, _)| key == &id)
//                 .unwrap_or((id, MemberCollection::new()));

//             let mut mapped_members: HashMap<Principal, Join> = HashMap::new();

//             for member_principal in group_members.1.get_member_principals() {
//                 let member = proxy_members
//                     .clone()
//                     .into_iter()
//                     .find(|(key, _)| key == &member_principal);

//                 if let Some((_, member)) = member {
//                     let joined = member.get_joined(&id);
//                     if let Some(joined) = joined {
//                         mapped_members.insert(
//                             member_principal,
//                             Join {
//                                 roles: joined.roles,
//                                 updated_at: joined.updated_at,
//                                 created_at: joined.created_at,
//                             },
//                         );
//                     }
//                 }
//             }

//             let mut mapped_invites: HashMap<Principal, Invite> = HashMap::new();

//             for invite_principal in group_members.1.get_invite_principals() {
//                 let invitee = proxy_members
//                     .clone()
//                     .into_iter()
//                     .find(|(key, _)| key == &invite_principal);

//                 if let Some((_, invitee)) = invitee {
//                     let invite = invitee.get_invite(&id);
//                     if let Some(invite) = invite {
//                         mapped_invites.insert(
//                             invite_principal,
//                             Invite {
//                                 invite_type: invite.invite_type,
//                                 notification_id: invite.notification_id,
//                                 updated_at: invite.updated_at,
//                                 created_at: invite.created_at,
//                             },
//                         );
//                     }
//                 }
//             }

//             let x = GroupMapArgs {
//                 group: group.clone(),
//                 events: events.1.get_event_ids(),
//                 members: Members {
//                     members: mapped_members,
//                     invites: mapped_invites,
//                     special_members: group.special_members,
//                     roles: group.roles,
//                 },
//             };

//             mapped.push((id, x.into()));
//         }

//         migrate("groups", mapped, None).await
//     }
// }
