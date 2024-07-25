use crate::{
    canister_methods::Canister,
    result::CanisterResult,
    utils::{Environment, AGENT},
};
use candid::{CandidType, Principal};
use eyre::Error;
use serde::de::DeserializeOwned;

pub struct IndexCalls {
    pub canister: Canister,
}

impl IndexCalls {
    pub fn new(canister: Canister) -> Self {
        Self { canister }
    }

    pub async fn insert<K: CandidType + DeserializeOwned, V: CandidType + DeserializeOwned>(
        &self,
        k: K,
        v: V,
    ) -> eyre::Result<(K, V), Error> {
        let response = self.canister.update("insert", (k, v)).await?;
        CanisterResult::try_from(response.as_slice())?.into_result()
    }
}

pub fn index_by_environment() -> Indexes {
    match AGENT.1 {
        Environment::Development => Indexes {
            profiles: Canister::new("qj423-uyaaa-aaaap-aho4a-cai"),
            // temporary principals
            groups: Canister::new(Principal::anonymous().to_string().as_str()),
            events: Canister::new(Principal::anonymous().to_string().as_str()),
            members: Canister::new(Principal::anonymous().to_string().as_str()),
            attendees: Canister::new(Principal::anonymous().to_string().as_str()),
            notifications: Canister::new(Principal::anonymous().to_string().as_str()),
            reports: Canister::new(Principal::anonymous().to_string().as_str()),
            friend_requests: Canister::new(Principal::anonymous().to_string().as_str()),
        },
        Environment::Staging => panic!("Staging not implemented"),
        Environment::Production => panic!("Production not implemented"),
    }
}

// should be extended with more indexes once available
pub struct Indexes {
    pub profiles: Canister,
    pub groups: Canister,
    pub events: Canister,
    pub members: Canister,
    pub attendees: Canister,
    pub notifications: Canister,
    pub reports: Canister,
    pub friend_requests: Canister,
}
