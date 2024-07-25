use crate::{
    index_methods::{IndexCallsTrait, ProfileIndexCalls},
    proxy_methods::ProxyCalls,
};

pub async fn migrate_profiles() -> eyre::Result<(u32, u32)> {
    let proxy_profiles = ProxyCalls::get_profiles().await?;

    println!("Migrating {} profiles", proxy_profiles.len());

    let mut success: u32 = 0;
    let mut failed: u32 = 0;

    for (principal, profile) in &proxy_profiles {
        match ProfileIndexCalls::insert(*principal, profile.clone()).await {
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
