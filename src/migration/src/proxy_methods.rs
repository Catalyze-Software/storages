use candid::Decode;
use catalyze_shared::profile::ProfileEntry;

use crate::{
    index_methods::Canister,
    utils::{context, Context, Environment},
};

pub struct Proxy(Context);

impl Proxy {
    pub fn new(env: Environment) -> Self {
        Self(context(env))
    }

    pub async fn get_profiles(&self) -> eyre::Result<Vec<ProfileEntry>> {
        let ctx = &self.0;
        let response: Vec<u8> = ctx.proxy.query("mig_profiles_get_all", None).await?;
        Ok(Decode!(&response, Vec<ProfileEntry>)
            .expect("Failed to decode mig_profiles_get_all response"))
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

pub fn proxy_by_environment(env: &Environment) -> Canister {
    match env {
        Environment::Development => Canister::new("bwm3m-wyaaa-aaaag-qdiua-cai"),
        Environment::Staging => panic!("Staging not implemented"),
        Environment::Production => panic!("Production not implemented"),
        // Environment::Staging => Principal::from_text("24swh-4iaaa-aaaap-ahevq-cai").unwrap(),
        // Environment::Production => Principal::from_text("2jvhk-5aaaa-aaaap-ahewa-cai").unwrap(),
    }
}
