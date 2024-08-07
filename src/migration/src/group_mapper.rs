use catalyze_shared::{
    general_structs::{
        members::Members, metadata::Metadata, privacy::Privacy, references::References,
    },
    group::Group,
    group_with_members::GroupWithMembers,
};

pub fn map_group(group: Group, members: Members, events: Vec<u64>) -> GroupWithMembers {
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
