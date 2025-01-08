use actix_web::{web, App, HttpRequest, HttpServer};

use tracing::{info, span, Level};

async fn hello(_req: HttpRequest) -> &'static str {
    let span = span!(Level::INFO, "hello function");
    let _guard = span.enter();
    info!("before returning Hello");
    "Hello"
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    tracing_subscriber::fmt::init();

    info!("starting...");

    HttpServer::new(|| App::new().service(web::resource("/").to(hello)))
        .bind(("0.0.0.0", 8080))?
        .run()
        .await
}
