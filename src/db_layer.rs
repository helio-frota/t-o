use anyhow::{Result, anyhow};
use rand::random;
use std::time::Duration;
use tokio::time::sleep;
use tracing::instrument;

// #[instrument(skip_all, err)]
pub async fn db_stuff() -> Result<()> {
    let duration = Duration::from_millis(1500);
    sleep(duration).await;

    if random::<bool>() {
        Err(anyhow!("simulated db error"))?;
    }

    Ok(())
}
