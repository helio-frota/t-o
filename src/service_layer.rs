use tokio::time::{Duration, sleep};

use crate::db_layer::db_stuff;
use tracing::Level;

#[derive(Debug)]
pub enum ServiceError {
    SimulatedFailure,
}

// #[instrument(skip_all)]
// #[instrument(skip_all, level = "trace")]
pub async fn service_stuff() -> Result<(), ServiceError> {
    let duration = Duration::from_millis(300);
    sleep(duration).await;

    let should_fail = false;

    if should_fail {
        let span = tracing::span!(Level::ERROR, "service_stuff");
        let _ = span.enter();
        tracing::error!("service_stuff failed: simulated failure");
        return Err(ServiceError::SimulatedFailure);
    }

    db_stuff().await;
    Ok(())
}
