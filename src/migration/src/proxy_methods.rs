use candid::{Decode, Principal};
use catalyze_shared::{group::Group, profile::Profile};

use crate::{
    canister_methods::Canister,
    utils::{context, Environment, AGENT},
};

pub struct ProxyCalls;

impl ProxyCalls {
    pub async fn get_profiles() -> eyre::Result<Vec<(Principal, Profile)>> {
        let response = context().proxy.query("mig_profiles_get_all", ()).await?;
        println!("Response: {:?}", response);
        Ok(Decode!(&response, Vec<(Principal, Profile)>)
            .expect("Failed to decode mig_profiles_get_all response"))
    }

    pub async fn get_groups(&self) -> eyre::Result<Vec<(u64, Group)>> {
        let response = context().proxy.query("mig_groups_get_all", ()).await?;
        println!("Response: {:?}", response);
        Ok(Decode!(&response, Vec<(u64, Group)>)
            .expect("Failed to decode mig_groups_get_all response"))
    }

    // Implement when idexes / shards are ready
    // mig_groups_get_all
    // mig_events_get_all
    // mig_reports_get_all
    // mig_members_get_all
    // mig_attendee_get_all
    // mig_friend_requests_get_all
    // mig_boosted_get_all
    // mig_notifications_get_all
    // mig_user_notifications_get_all
    // mig_group_members_get_all
    // mig_event_attendees_get_all
    // mig_group_events_get_all
    // mig_tags_get_all
    // mig_categories_get_all
    // mig_skills_get_all
}

pub fn proxy_by_environment() -> Canister {
    match AGENT.1 {
        Environment::Development => Canister::new("bwm3m-wyaaa-aaaag-qdiua-cai"),
        Environment::Staging => panic!("Staging not implemented"),
        Environment::Production => panic!("Production not implemented"),
        // Environment::Staging => Principal::from_text("24swh-4iaaa-aaaap-ahevq-cai").unwrap(),
        // Environment::Production => Principal::from_text("2jvhk-5aaaa-aaaap-ahewa-cai").unwrap(),
    }
}
