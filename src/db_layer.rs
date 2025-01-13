use tokio::time::{sleep, Duration};

use tracing::instrument;

#[instrument(skip_all)]
pub async fn db_stuff() {
    let duration = Duration::from_millis(1500);
    sleep(duration).await;
}
