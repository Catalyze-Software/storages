use crate::{
    canister_methods::Canister,
    utils::{Environment, AGENT},
};
use candid::CandidType;
use eyre::Error;
use serde::{de::DeserializeOwned, Serialize};

pub struct IndexCalls {
    pub canister: Canister,
}

impl IndexCalls {
    pub fn new(canister: Canister) -> Self {
        Self { canister }
    }

    pub async fn insert<
        K: CandidType + Serialize + DeserializeOwned,
        V: CandidType + Serialize + DeserializeOwned,
    >(
        &self,
        k: K,
        v: V,
    ) -> eyre::Result<(), Error> {
        self.canister.update("insert", k, v).await?;
        Ok(())
        // CanisterResult::try_from(response.as_slice())?.into_result()
    }
}

pub fn index_by_environment() -> Indexes {
    match AGENT.environment {
        Environment::Development => Indexes {
            profiles: Canister::new("qj423-uyaaa-aaaap-aho4a-cai"),
            groups: Canister::new("ivgkz-aqaaa-aaaap-ahtaa-cai"),
            events: Canister::new("i4fbf-wyaaa-aaaap-ahtbq-cai"),
            notifications: Canister::new("xz5up-6qaaa-aaaap-ahwxq-cai"),
            reports: Canister::new("iha5a-maaaa-aaaap-ahtda-cai"),
            friend_requests: Canister::new("vwgn6-biaaa-aaaap-ahw3a-cai"),
            boosted: Canister::new("vyeaw-2yaaa-aaaap-ahw2a-cai"),
            topic: Canister::new("v7fgc-xaaaa-aaaap-ahw2q-cai"),
        },
        Environment::Staging => panic!("Staging not implemented"),
        Environment::Production => panic!("Production not implemented"),
    }
}

pub struct Indexes {
    pub profiles: Canister,
    pub groups: Canister,
    pub events: Canister,
    pub notifications: Canister,
    pub reports: Canister,
    pub friend_requests: Canister,
    pub boosted: Canister,
    pub topic: Canister,
}
