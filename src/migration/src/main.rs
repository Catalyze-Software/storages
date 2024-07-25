use std::env;

use migration::migrate_methods;

#[tokio::main]
pub async fn main() -> eyre::Result<()> {
    env::set_var("ENV", "development");

    let profiles = migrate_methods::migrate_profiles().await?;
    println!("Profiles: {:?}", profiles);

    Ok(())
}
