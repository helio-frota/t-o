use actix_web::{web, App, HttpRequest, HttpServer};

use tracing::{debug, error, info, span, trace, warn};

async fn hello(_req: HttpRequest) -> &'static str {
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
