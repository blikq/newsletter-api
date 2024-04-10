use actix_web::*;
use std::{sync::Mutex, future::IntoFuture};
mod handlers;

#[get("/health_check")]
async fn health_check() -> impl Responder {
    handlers::health_check::main()
}



pub async fn run() -> std::io::Result<()> {
    
    HttpServer::new(move || App::new()
    .service(health_check)
    )
    .bind("127.0.0.1:8080")?
    .run()
    .await

}