use std::{path::Path, str::FromStr};

use candid::Principal;
use elliptic_curve::SecretKey;
use ic_agent::{
    identity::{BasicIdentity, Secp256k1Identity},
    Identity,
};

pub fn random_principal() -> Principal {
    let private_key = SecretKey::random(&mut rand::thread_rng());
    Secp256k1Identity::from_private_key(private_key)
        .sender()
        .expect("Failed to get sender")
}

pub struct Context {
    pub agent: ic_agent::Agent,
    pub index_profile: Principal,
}

fn default_pem_path() -> String {
    let home_dir = std::env::var("HOME").expect("HOME environment variable is not set");

    format!(
        "{}/.config/dfx/identity/catalyze_development/identity.pem",
        home_dir
    )
}

pub async fn context() -> Context {
    let ic_url = std::env::var("IC_URL").unwrap_or_else(|_| "http://localhost:4943".to_string());
    let identity_path = std::env::var("IDENTITY_PATH").unwrap_or_else(|_| default_pem_path());
    let identity_path = Path::new(&identity_path);
    let identity = BasicIdentity::from_pem_file(identity_path).expect("Failed to get identity");
    let index_profile = std::env::var("INDEX_PROFILE")
        .unwrap_or_else(|_| "bnz7o-iuaaa-aaaaa-qaaaa-cai".to_string());
    let index_profile =
        Principal::from_str(&index_profile).expect("Failed to parse index profile principal");

    let agent = ic_agent::Agent::builder()
        .with_url(ic_url)
        .with_identity(identity)
        .build()
        .expect("Failed to build agent");

    agent
        .fetch_root_key()
        .await
        .expect("Failed to fetch root key for the icp agent");

    Context {
        index_profile,
        agent,
    }
}
