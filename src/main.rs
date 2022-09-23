use crate::scheduler::start_scheduler;
use actix_web::{get, App, HttpResponse, HttpServer, Result};

mod scheduler;

#[get("/health")]
pub async fn health() -> Result<HttpResponse> {
    Ok(HttpResponse::Ok().body("success".to_string()))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Start scheduler on a new thread
    actix_rt::spawn(async move {
        start_scheduler().await;
    });

    // Start Web server
    HttpServer::new(|| App::new().service(health))
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
