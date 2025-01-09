use actix_web::{web, App, HttpRequest, HttpServer};

use tracing::{info, span, Level};

use std::sync::Once;

mod otel;
use crate::otel::init_otel_traces;

async fn hello(_req: HttpRequest) -> &'static str {
    let span = span!(Level::INFO, "hello function");
    let _guard = span.enter();
    info!("before returning Hello");
    "Hello"
}

static INIT: Once = Once::new();

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // NOTE: for local experiments
    // tracing_subscriber::fmt::init();

    INIT.call_once(|| init_otel_traces("t-o"));

    info!("starting...");

    HttpServer::new(|| App::new().service(web::resource("/").to(hello)))
        .bind(("0.0.0.0", 8080))?
        .run()
        .await
}
