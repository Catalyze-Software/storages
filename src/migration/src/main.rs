use std::env;

use migration::migrate_methods::Migrate;

#[tokio::main]
pub async fn main() -> eyre::Result<()> {
    env::set_var("ENV", "development");

    Migrate::all().await?;

    Ok(())
}
