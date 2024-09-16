use crate::utils::AGENT;
use candid::{encode_args, CandidType, Encode, Principal};
use eyre::Context as _;
use serde::{de::DeserializeOwned, Serialize};

pub struct Canister {
    principal: Principal,
}

impl Canister {
    pub fn new(principal: &str) -> Self {
        Self {
            principal: Principal::from_text(principal).expect("Failed to parse principal"),
        }
    }

    pub async fn query<T: CandidType + DeserializeOwned>(
        &self,
        method: &str,
        args: T,
    ) -> eyre::Result<Vec<u8>> {
        AGENT
            .query_agent
            .query(&self.principal, method)
            .with_arg(Encode!(&args)?)
            .await
            .wrap_err_with(|| format!("Failed to perform query \"{}\" request", method))
    }

    pub async fn update<
        K: CandidType + Serialize + DeserializeOwned,
        V: CandidType + Serialize + DeserializeOwned,
    >(
        &self,
        method: &str,
        key: K,
        value: V,
    ) -> eyre::Result<()> {
        AGENT
            .migration_agent
            .update(&self.principal, method)
            .with_arg(encode_args((key, value))?)
            .call_and_wait()
            .await
            .wrap_err_with(|| format!("Failed to perform query \"{}\" request", method))?;

        Ok(())
    }
}
