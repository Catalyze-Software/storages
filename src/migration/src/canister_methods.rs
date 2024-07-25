use crate::utils::AGENT;
use candid::{CandidType, Encode, Principal};
use eyre::Context as _;

pub struct Canister {
    principal: Principal,
}

impl Canister {
    pub fn new(principal: &str) -> Self {
        Self {
            principal: Principal::from_text(principal).expect("Failed to parse principal"),
        }
    }

    pub async fn query<T: CandidType>(&self, method: &str, args: T) -> eyre::Result<Vec<u8>> {
        let response = AGENT
            .0
            .query(&self.principal, method)
            .with_arg(Encode!(&args)?)
            .await
            .wrap_err_with(|| format!("Failed to perform query \"{}\" request", method))?;

        Ok(response)
    }

    pub async fn update<T: CandidType>(&self, method: &str, args: T) -> eyre::Result<Vec<u8>> {
        let response = AGENT
            .0
            .update(&self.principal, method)
            .with_arg(Encode!(&args)?)
            .await
            .wrap_err_with(|| format!("Failed to perform query \"{}\" request", method))?;

        Ok(response)
    }
}
