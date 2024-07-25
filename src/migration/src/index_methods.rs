use std::future::Future;

use crate::{
    canister_methods::Canister,
    utils::{context, Environment, AGENT},
};
use candid::{CandidType, Principal};
use catalyze_shared::{group::Group, profile::Profile};
use eyre::Error;

pub trait IndexCallsTrait<K: CandidType, V: CandidType> {
    fn insert(k: K, v: V) -> impl Future<Output = Result<Vec<u8>, Error>> + Send;
}

pub struct ProfileIndexCalls;
pub struct GroupIndexCalls;

impl IndexCallsTrait<Principal, Profile> for ProfileIndexCalls {
    async fn insert(k: Principal, v: Profile) -> Result<Vec<u8>, Error> {
        context().indexes.profiles.update("insert", (k, v)).await
    }
}

impl IndexCallsTrait<u64, Group> for GroupIndexCalls {
    async fn insert(k: u64, v: Group) -> Result<Vec<u8>, Error> {
        context().indexes.groups.update("insert", (k, v)).await
    }
}

pub fn index_by_environment() -> Indexes {
    match AGENT.1 {
        Environment::Development => Indexes {
            profiles: Canister::new("qj423-uyaaa-aaaap-aho4a-cai"),
            // temporary principal
            groups: Canister::new(Principal::anonymous().to_string().as_str()),
        },
        Environment::Staging => panic!("Staging not implemented"),
        Environment::Production => panic!("Production not implemented"),
    }
}

// should be extended with more indexes once available
pub struct Indexes {
    pub profiles: Canister,
    pub groups: Canister,
}
