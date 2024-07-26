use candid::{Decode, Principal};
use catalyze_shared::{
    attendee::Attendee, boosted::Boost, event::Event, event_collection::EventCollection,
    friend_request::FriendRequest, group::Group, member::Member,
    member_collection::MemberCollection, notification::Notification, profile::Profile,
    report::Report, user_notifications::UserNotifications,
};

use crate::{
    canister_methods::Canister,
    utils::{context, Environment, AGENT},
};

pub struct ProxyCalls;

impl ProxyCalls {
    pub async fn get_profiles() -> eyre::Result<Vec<(Principal, Profile)>> {
        let response = context().proxy.query("mig_profiles_get_all", ()).await?;
        Ok(Decode!(&response, Vec<(Principal, Profile)>)
            .expect("Failed to decode mig_profiles_get_all response"))
    }

    pub async fn get_groups() -> eyre::Result<Vec<(u64, Group)>> {
        let response = context().proxy.query("mig_groups_get_all", ()).await?;
        Ok(Decode!(&response, Vec<(u64, Group)>)
            .expect("Failed to decode mig_groups_get_all response"))
    }

    pub async fn get_events() -> eyre::Result<Vec<(u64, Event)>> {
        let response = context().proxy.query("mig_events_get_all", ()).await?;
        Ok(Decode!(&response, Vec<(u64, Event)>)
            .expect("Failed to decode mig_events_get_all response"))
    }

    pub async fn get_reports() -> eyre::Result<Vec<(u64, Report)>> {
        let response = context().proxy.query("mig_reports_get_all", ()).await?;
        Ok(Decode!(&response, Vec<(u64, Report)>)
            .expect("Failed to decode mig_reports_get_all response"))
    }

    pub async fn get_members() -> eyre::Result<Vec<(Principal, Member)>> {
        let response = context().proxy.query("mig_members_get_all", ()).await?;
        Ok(Decode!(&response, Vec<(Principal, Member)>)
            .expect("Failed to decode mig_members_get_all response"))
    }

    pub async fn get_attendees() -> eyre::Result<Vec<(Principal, Attendee)>> {
        let response = context().proxy.query("mig_attendee_get_all", ()).await?;
        Ok(Decode!(&response, Vec<(Principal, Attendee)>)
            .expect("Failed to decode mig_attendee_get_all response"))
    }

    pub async fn get_friend_requests() -> eyre::Result<Vec<(u64, FriendRequest)>> {
        let response = context()
            .proxy
            .query("mig_friend_requests_get_all", ())
            .await?;
        Ok(Decode!(&response, Vec<(u64, FriendRequest)>)
            .expect("Failed to decode mig_friend_requests_get_all response"))
    }

    pub async fn get_boosted() -> eyre::Result<Vec<(u64, Boost)>> {
        let response = context().proxy.query("mig_boosted_get_all", ()).await?;
        Ok(Decode!(&response, Vec<(u64, Boost)>)
            .expect("Failed to decode mig_boosted_get_all response"))
    }

    pub async fn get_notifications() -> eyre::Result<Vec<(u64, Notification)>> {
        let response = context()
            .proxy
            .query("mig_notifications_get_all", ())
            .await?;
        Ok(Decode!(&response, Vec<(u64, Notification)>)
            .expect("Failed to decode mig_notifications_get_all response"))
    }

    pub async fn get_user_notifications() -> eyre::Result<Vec<(Principal, UserNotifications)>> {
        let response = context()
            .proxy
            .query("mig_user_notifications_get_all", ())
            .await?;
        Ok(Decode!(&response, Vec<(Principal, UserNotifications)>)
            .expect("Failed to decode mig_user_notifications_get_all response"))
    }

    pub async fn get_group_members() -> eyre::Result<Vec<(u64, MemberCollection)>> {
        let response = context()
            .proxy
            .query("mig_group_members_get_all", ())
            .await?;
        Ok(Decode!(&response, Vec<(u64, MemberCollection)>)
            .expect("Failed to decode mig_group_members_get_all response"))
    }

    pub async fn get_event_attendees() -> eyre::Result<Vec<(u64, MemberCollection)>> {
        let response = context()
            .proxy
            .query("mig_event_attendees_get_all", ())
            .await?;
        Ok(Decode!(&response, Vec<(u64, MemberCollection)>)
            .expect("Failed to decode mig_event_attendees_get_all response"))
    }

    pub async fn get_group_events() -> eyre::Result<Vec<(u64, EventCollection)>> {
        let response = context()
            .proxy
            .query("mig_group_events_get_all", ())
            .await?;
        Ok(Decode!(&response, Vec<(u64, EventCollection)>)
            .expect("Failed to decode mig_group_events_get_all response"))
    }

    pub async fn get_tags() -> eyre::Result<Vec<(u64, String)>> {
        let response = context().proxy.query("mig_tags_get_all", ()).await?;
        Ok(Decode!(&response, Vec<(u64, String)>)
            .expect("Failed to decode mig_tags_get_all response"))
    }

    pub async fn get_categories() -> eyre::Result<Vec<(u64, String)>> {
        let response = context().proxy.query("mig_categories_get_all", ()).await?;
        Ok(Decode!(&response, Vec<(u64, String)>)
            .expect("Failed to decode mig_categories_get_all response"))
    }

    pub async fn get_skills() -> eyre::Result<Vec<(u64, String)>> {
        let response = context().proxy.query("mig_skills_get_all", ()).await?;
        Ok(Decode!(&response, Vec<(u64, String)>)
            .expect("Failed to decode mig_skills_get_all response"))
    }
}

pub fn proxy_by_environment() -> Canister {
    match AGENT.1 {
        Environment::Development => Canister::new("bwm3m-wyaaa-aaaag-qdiua-cai"),
        // Environment::Staging => Principal::from_text("24swh-4iaaa-aaaap-ahevq-cai").unwrap(),
        Environment::Staging => panic!("Staging not implemented"),
        // Environment::Production => Principal::from_text("2jvhk-5aaaa-aaaap-ahewa-cai").unwrap(),
        Environment::Production => panic!("Production not implemented"),
    }
}
