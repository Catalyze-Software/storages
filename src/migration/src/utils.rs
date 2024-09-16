use lazy_static::lazy_static;
use std::sync::Arc;

use ic_agent::{
    identity::{BasicIdentity, Secp256k1Identity},
    Agent,
};

use crate::{
    canister_methods::Canister,
    index_methods::{index_by_environment, Indexes},
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

pub struct AgentData {
    pub query_agent: Agent,
    pub migration_agent: Agent,
    pub environment: Environment,
}

lazy_static! {
    pub static ref AGENT: Arc<AgentData> = {
        let environment: Environment = match std::env::var("ENV") {
            Ok(env) => match env.as_str() {
                "development" => Environment::Development,
                "staging" => Environment::Staging,
                "production" => Environment::Production,
                _ => Environment::Development,
            },
            Err(_) => panic!("No environment set"),
        };
        let ic_url = "https://icp0.io/";
        let home_dir = std::env::var("HOME").expect("HOME environment variable is not set");

        let query_identity_path = format!(
            "{}/.config/dfx/identity/catalyze_{}/identity.pem",
            home_dir,
            std::env::var("ENV").expect("ENV environment variable is not set")
        );

        // development / staging / production
        let migration_identity_path = format!(
            "{}/.config/dfx/identity/catalyze_migration/identity.pem",
            home_dir,
        );


        let query_identity = BasicIdentity::from_pem_file(query_identity_path).expect("Failed to get identity");
        let migration_identity = Secp256k1Identity::from_pem_file(migration_identity_path).expect("Failed to get identity");

        let query_agent = Agent::builder()
            .with_url(ic_url)
            .with_identity(query_identity)
            .build()
            .expect("Failed to build agent");

        let migration_agent = Agent::builder()
            .with_url(ic_url)
            .with_identity(migration_identity)
            .build()
            .expect("Failed to build agent");

        Arc::new(AgentData {
            query_agent,
            migration_agent,
            environment
        })

        // here for reference and local testing

    };
}

pub fn context() -> Context {
    Context {
        indexes: index_by_environment(),
        proxy: proxy_by_environment(),
    }
}
