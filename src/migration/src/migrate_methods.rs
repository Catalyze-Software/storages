use std::{fmt::Debug, future::Future};

use candid::CandidType;
use serde::de::DeserializeOwned;

use crate::{canister_methods::Canister, index_methods::IndexCalls, proxy_methods::ProxyCalls};

pub struct Migrate;

impl Migrate {
    pub async fn profiles() -> eyre::Result<(u32, u32)> {
        migrate(
            "profiles",
            ProxyCalls::get_profiles(),
            None, // Some(context().indexes.profiles),
        )
        .await
    }

    pub async fn groups() -> eyre::Result<(u32, u32)> {
        migrate("groups", ProxyCalls::get_groups(), None).await
    }

    pub async fn events() -> eyre::Result<(u32, u32)> {
        migrate("events", ProxyCalls::get_events(), None).await
    }

    pub async fn reports() -> eyre::Result<(u32, u32)> {
        migrate("reports", ProxyCalls::get_reports(), None).await
    }

    pub async fn members() -> eyre::Result<(u32, u32)> {
        migrate("members", ProxyCalls::get_members(), None).await
    }

    pub async fn attendees() -> eyre::Result<(u32, u32)> {
        migrate("attendees", ProxyCalls::get_attendees(), None).await
    }

    pub async fn friend_requests() -> eyre::Result<(u32, u32)> {
        migrate("friend_requests", ProxyCalls::get_friend_requests(), None).await
    }

    pub async fn notifications() -> eyre::Result<(u32, u32)> {
        migrate("notifications", ProxyCalls::get_notifications(), None).await
    }

    pub async fn all() -> eyre::Result<()> {
        let profiles = Migrate::profiles().await?;
        let groups = Migrate::groups().await?;
        let events = Migrate::events().await?;
        let reports = Migrate::reports().await?;
        let members = Migrate::members().await?;
        let attendees = Migrate::attendees().await?;
        let friend_requests = Migrate::friend_requests().await?;
        let notifications = Migrate::notifications().await?;

        println!(
            "Migration completed: profiles: {:?}, groups: {:?}, events: {:?}, reports: {:?}, members: {:?}, attendees: {:?}, friend_requests: {:?}, notifications: {:?}",
            profiles, groups, events, reports, members, attendees, friend_requests, notifications
        );

        Ok(())
    }
}

pub async fn migrate<K, V, C>(
    reference: &str,
    proxy_callback: C,
    index: Option<Canister>,
) -> eyre::Result<(u32, u32)>
where
    K: CandidType + Clone + DeserializeOwned + Debug,
    V: CandidType + Clone + DeserializeOwned + Debug,
    C: Future<Output = eyre::Result<Vec<(K, V)>>> + Send,
{
    let data = proxy_callback.await?;
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
                    println!("Failed to migrate item {:?}: {:?}", key, e);
                    failed += 1;
                }
            }
        }
    } else {
        println!("Skipping migration of {} {}", data.len(), reference);
    }
    Ok((success, failed))
}
