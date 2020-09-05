use crate::{
    scheduler::Scheduler
};
use actix_web::{get, App, HttpServer, HttpResponse, Result};
use actix::prelude::*;

mod scheduler;

#[get("/health")]
pub async fn health() -> Result<HttpResponse> {
    Ok(HttpResponse::Ok().body("success".to_string()))
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {

    // Start Scheduler
    Scheduler.start();

    // Start Web server
    HttpServer::new(|| App::new().service(health))
        .bind("127.0.0.1:8080")?
        .run()
        .await
}