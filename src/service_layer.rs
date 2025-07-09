use anyhow::{Context, Result};
use tracing::Level;
use tracing::instrument;

use crate::db_layer::db_stuff;

// NOTE: This is working when using manual span creation
// We can have error only for this and continuing using INFO for all the rest
pub async fn service_stuff() -> Result<()> {
    match db_stuff().await {
        Ok(_) => Ok(()),
        Err(e) => {
            let span = tracing::error_span!("service_stuff_error", error = %e);
            let _ = span.enter();
            Err(e)
        }
    }
}

// NOTE: This is creating spans with or without error
// Also the error is weirdly shown as INFO
// #[instrument(skip_all, level = Level::ERROR)]
// pub async fn service_stuff() -> Result<()> {
//     db_stuff().await?;
//
//     Ok(())
// }

#[instrument(skip_all)]
pub async fn other_foo_bar() {
    print!("other_foo_bar in the same service_sfuff namespace");
}
