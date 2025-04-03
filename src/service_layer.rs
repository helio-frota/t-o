use tokio::time::{Duration, sleep};

use crate::db_layer::db_stuff;
use tracing::instrument;

#[instrument(skip_all)]
pub async fn service_stuff() {
    let duration = Duration::from_millis(300);
    sleep(duration).await;
    db_stuff().await;
}
