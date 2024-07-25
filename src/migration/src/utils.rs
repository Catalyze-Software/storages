use lazy_static::lazy_static;
use std::sync::Arc;

use ic_agent::{identity::BasicIdentity, Agent};

use crate::{
    index_methods::{index_by_environment, Canister, Indexes},
    proxy_methods::proxy_by_environment,
};

pub struct Context {
    pub indexes: Indexes,
    pub proxy: Canister,
}

pub enum Environment {
    Development,
    Staging,
    Production,
}

lazy_static! {
    pub static ref AGENT: Arc<Agent> = {
        let environment: Environment = match std::env::var("ENV") {
            Ok(env) => match env.as_str() {
                "development" => Environment::Development,
                "staging" => Environment::Staging,
                "production" => Environment::Production,
                _ => Environment::Development,
            },
            Err(_) => panic!("No environment set"),
        };
        let ic_url = "https://icp0.io";
        let identity_path = default_pem_path(&environment); // Default to development
        let identity = BasicIdentity::from_pem_file(identity_path).expect("Failed to get identity");

        Arc::new(
            Agent::builder()
                .with_url(ic_url)
                .with_identity(identity)
                .build()
                .expect("Failed to build agent"),
        )

        // here for reference and local testing
        // agent
        //     .fetch_root_key()
        //     .await
        //     .expect("Failed to fetch root key for the icp agent");
    };
}

fn default_pem_path(env: &Environment) -> String {
    let home_dir = std::env::var("HOME").expect("HOME environment variable is not set");

    match env {
        Environment::Development => format!(
            "{}/.config/dfx/identity/catalyze_development/identity.pem",
            home_dir
        ),
        Environment::Staging => format!(
            "{}/.config/dfx/identity/catalyze_staging/identity.pem",
            home_dir
        ),
        Environment::Production => format!(
            "{}/.config/dfx/identity/catalyze_production/identity.pem",
            home_dir
        ),
    }
}

pub fn context(env: Environment) -> Context {
    Context {
        indexes: index_by_environment(&env),
        proxy: proxy_by_environment(&env),
    }
}
