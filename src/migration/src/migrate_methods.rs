use candid::Principal;
use catalyze_shared::profile::Profile;

use crate::{index_methods::IndexCalls, proxy_methods::ProxyCalls, utils::context};

pub async fn migrate_profiles() -> eyre::Result<(u32, u32)> {
    let proxy_profiles = ProxyCalls::get_profiles().await?;

    println!("Migrating {} profiles", proxy_profiles.len());

    let mut success: u32 = 0;
    let mut failed: u32 = 0;

    let index = IndexCalls::new(context().indexes.profiles);

    for (principal, profile) in &proxy_profiles {
        match index
            .insert::<Principal, Profile>(*principal, profile.clone())
            .await
        {
            Ok(_) => {
                println!("Profile {:?} migrated successfully", principal);
                success += 1;
            }
            Err(e) => {
                println!("Failed to migrate profile {:?}: {:?}", principal, e);
                failed += 1;
            }
        }
    }
    Ok((success, failed))
}
