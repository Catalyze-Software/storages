use std::collections::HashMap;

use catalyze_shared::{
    general_structs::privacy::Privacy,
    privacy::PrivacyType,
    profile::Profile,
    profile_structs::{
        profile_documents::ProfileDocuments, profile_metadata::ProfileMetadata,
        profile_references::ProfileReferences,
    },
    profile_with_refs::ProfileWithRefs,
};

pub struct ProfileMapArgs {
    pub profile: Profile,
    pub groups: Vec<u64>,
    pub events: Vec<u64>,
}

impl From<ProfileMapArgs> for ProfileWithRefs {
    fn from(args: ProfileMapArgs) -> Self {
        let ProfileMapArgs {
            profile,
            groups,
            events,
        } = args;
        ProfileWithRefs {
            metadata: ProfileMetadata {
                username: profile.username,
                about: profile.about,
                banner_image: profile.banner_image,
                city: profile.city,
                country: profile.country,
                date_of_birth: profile.date_of_birth,
                email: profile.email,
                first_name: profile.first_name,
                display_name: profile.display_name,
                last_name: profile.last_name,
                profile_image: profile.profile_image,
                state_or_province: profile.state_or_province,
                website: profile.website,
            },
            application_role: profile.application_role,
            created_on: profile.created_on,
            documents: ProfileDocuments {
                code_of_conduct: profile.code_of_conduct,
                terms_of_service: profile.terms_of_service,
                privacy_policy: profile.privacy_policy,
            },
            extra: if profile.extra.is_empty() {
                None
            } else {
                Some(profile.extra)
            },
            notification_id: profile.notification_id,
            privacy: Privacy {
                privacy_type: match profile.privacy {
                    catalyze_shared::profile_privacy::ProfilePrivacy::Private => {
                        PrivacyType::Private
                    }
                    catalyze_shared::profile_privacy::ProfilePrivacy::Public => PrivacyType::Public,
                },
                privacy_gated_type_amount: None,
            },
            references: ProfileReferences {
                groups,
                events,
                causes: profile.causes,
                interests: profile.interests,
                pinned: profile.pinned,
                relations: profile.relations,
                skills: profile.skills,
                starred: profile.starred,
                wallets: HashMap::from_iter(
                    profile
                        .wallets
                        .iter()
                        .map(|(principal, wallet)| (principal.to_string(), wallet.clone())),
                ),
            },
            updated_on: profile.updated_on,
        }
    }
}
