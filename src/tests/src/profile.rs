use std::{collections::HashMap, time::SystemTime};

use catalyze_shared::{
    application_role::ApplicationRole,
    asset::Asset,
    profile::{Profile, ProfileFilter},
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
        let now = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_secs();

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
            updated_on: now,
            created_on: now,
        };

        profiles.push(profile);
    }

    let mut profile_ids = vec![];

    for profile in profiles.clone() {
        let now = std::time::Instant::now();

        let id = random_principal();

        let resp = calls::profile::insert(&ctx, (id, profile.clone()))
            .await
            .expect("Failed to insert profile");

        println!(
            "Inserted profile: ID: {}, name: {} elapsed: {:.2?}",
            resp.0,
            resp.1.username,
            now.elapsed()
        );

        let now = std::time::Instant::now();

        println!("Get profile by id: {}, name: {}", resp.0, resp.1.username);

        let resp = calls::profile::get(&ctx, resp.0)
            .await
            .expect("Failed to get profile");

        println!(
            "Got profile: ID: {}, name: {}, elapsed: {:.2?}\n",
            resp.0,
            resp.1.username,
            now.elapsed()
        );

        profile_ids.push(id);
    }

    let ids = &profile_ids[..3];

    println!(
        "Get many profiles by id: {:#?}",
        ids.iter().map(|id| id.to_string()).collect::<Vec<_>>()
    );

    let now = std::time::Instant::now();

    let resp = calls::profile::get_many(&ctx, ids.to_vec())
        .await
        .expect("Failed to get many profiles");

    println!(
        "Got many profiles: {:#?}, elapsed: {:.2?}\n",
        resp.iter()
            .map(|(id, profile)| (id.to_string(), profile.username.clone()))
            .collect::<Vec<_>>(),
        now.elapsed()
    );

    println!("Get all profiles");

    let now = std::time::Instant::now();

    let resp = calls::profile::get_all(&ctx)
        .await
        .expect("Failed to get all profiles");

    println!(
        "Got all profiles: {:#?}, elapsed: {:.2?}",
        resp.iter()
            .map(|(id, profile)| (id.to_string(), profile.username.clone()))
            .collect::<Vec<_>>(),
        now.elapsed()
    );

    println!("Find profile");

    let now = std::time::Instant::now();

    let resp = calls::profile::find(&ctx, vec![ProfileFilter::Username("user_1".to_owned())])
        .await
        .expect("Failed to find profile")
        .expect("Profile not found");

    println!(
        "Found profile: ID: {}, name: {}, elapsed: {:.2?}\n",
        resp.0,
        resp.1.username,
        now.elapsed()
    );

    println!("Filter profiles");

    let now = std::time::Instant::now();

    let resp = calls::profile::filter(&ctx, vec![ProfileFilter::Username("user_0".to_owned())])
        .await
        .expect("Failed to filter profile");

    println!(
        "Got filtered profiles: {:#?}, elapsed: {:.2?}",
        resp.iter()
            .map(|(id, profile)| (id.to_string(), profile.username.clone()))
            .collect::<Vec<_>>(),
        now.elapsed()
    );

    println!("Update profile");

    let now = std::time::Instant::now();

    let id = profile_ids[0];
    let mut profile = profiles[0].clone();

    profile.username = "user_0_updated".to_string();

    let resp = calls::profile::update(&ctx, (id, profile.clone()))
        .await
        .expect("Failed to update profile");

    println!(
        "Updated profile: ID: {}, name: {}, elapsed: {:.2?}\n",
        resp.0,
        resp.1.username,
        now.elapsed()
    );

    println!("Remove profile");

    let now = std::time::Instant::now();

    let id = profile_ids[0];

    calls::profile::remove(&ctx, id)
        .await
        .expect("Failed to remove profile");

    println!(
        "Removed profile: ID: {}, elapsed: {:.2?}\n",
        id,
        now.elapsed()
    );

    println!("Remove many profiles");

    let now = std::time::Instant::now();

    let ids = &profile_ids[1..3];

    calls::profile::remove_many(&ctx, ids.to_vec())
        .await
        .expect("Failed to remove many profiles");

    println!("Removed many profiles, elapsed: {:.2?}", now.elapsed());
}
