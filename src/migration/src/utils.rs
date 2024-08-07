use lazy_static::lazy_static;
use std::sync::Arc;

use ic_agent::{identity::BasicIdentity, Agent};

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

lazy_static! {
    pub static ref AGENT: Arc<(Agent, Environment)> = {
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

        // development / staging / production
        let identity_path = format!(
            "{}/.config/dfx/identity/catalyze_{}/identity.pem",
            home_dir,
            std::env::var("ENV").expect("ENV environment variable is not set")
        );

        let identity = BasicIdentity::from_pem_file(identity_path).expect("Failed to get identity");

        let agent = Agent::builder()
            .with_url(ic_url)
            .with_identity(identity)
            .build()
            .expect("Failed to build agent");

        Arc::new((
            agent,
            environment)
        )

        // here for reference and local testing

    };
}

pub fn context() -> Context {
    Context {
        indexes: index_by_environment(),
        proxy: proxy_by_environment(),
    }
}
