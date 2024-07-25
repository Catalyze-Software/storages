use crate::utils::{Environment, AGENT};
use candid::Principal;
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

    pub async fn query(&self, method: &str, args: Option<Vec<u8>>) -> eyre::Result<Vec<u8>> {
        let mut query = AGENT.query(&self.principal, method);

        if let Some(args) = args {
            query = query.with_arg(args);
        }

        let response = query
            .await
            .wrap_err_with(|| format!("Failed to perform query \"{}\" request", method))?;

        Ok(response)
    }

    pub async fn update(&self, method: &str, args: Option<Vec<u8>>) -> eyre::Result<Vec<u8>> {
        let mut update = AGENT.update(&self.principal, method);

        if let Some(args) = args {
            update = update.with_arg(args);
        }

        let response = update
            .await
            .wrap_err_with(|| format!("Failed to perform query \"{}\" request", method))?;

        Ok(response)
    }
}

pub fn index_by_environment(env: &Environment) -> Indexes {
    match env {
        Environment::Development => Indexes {
            profiles: Canister::new("qj423-uyaaa-aaaap-aho4a-cai"),
            groups: Canister::new("random"),
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
