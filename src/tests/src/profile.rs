use std::collections::HashMap;

use catalyze_shared::{
    application_role::ApplicationRole, asset::Asset, profile::Profile,
    profile_privacy::ProfilePrivacy,
};

use crate::{
    calls,
    utils::{context, random_principal},
};

#[tokio::test]
async fn test_insert_profiles() {
    let ctx = context().await;

    let mut profiles = vec![];

    for i in 0..10 {
        let name = format!("user_{i}");

        let profile: Profile = Profile {
            username: name.to_string(),
            display_name: name.to_string(),
            first_name: name.to_string(),
            last_name: name.to_string(),
            privacy: ProfilePrivacy::default(),
            extra: Default::default(),
            application_role: ApplicationRole::default(),
            about: "".to_owned(),
            email: "".to_owned(),
            date_of_birth: 0,
            city: "".to_owned(),
            state_or_province: "".to_owned(),
            country: "".to_owned(),
            profile_image: Asset::default(),
            banner_image: Asset::default(),
            skills: vec![],
            interests: vec![],
            causes: vec![],
            website: "".to_owned(),
            code_of_conduct: None,
            privacy_policy: None,
            terms_of_service: None,
            wallets: HashMap::new(),
            starred: vec![],
            pinned: vec![],
            relations: HashMap::new(),
            notification_id: None,
            updated_on: 0,
            created_on: 0,
        };

        profiles.push(profile);
    }

    for profile in profiles {
        let now = std::time::Instant::now();

        let resp = calls::profile::insert(&ctx, (random_principal(), profile.clone()))
            .await
            .expect("Failed to insert profile");

        println!(
            "Inserted profile: {:?}, elapsed: {:.2?}",
            resp,
            now.elapsed()
        );
    }
}
