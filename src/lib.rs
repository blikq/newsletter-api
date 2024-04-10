use actix_web::*;
use std::{sync::Mutex, future::IntoFuture};
mod handlers;

use serde::Deserialize;



#[get("/health_check")]
async fn health_check() -> impl Responder {
    handlers::health_check::main()
}

#[post("/suscribe")]
async fn suscribe(form: web::Form<handlers::suscribe::FormData>) -> impl Responder {
    handlers::suscribe::main(form)
}

pub async fn run() -> std::io::Result<()> {
    
    HttpServer::new(move || App::new()
    .service(health_check)
    .service(suscribe)
    )
    .bind("127.0.0.1:8080")?
    .run()
    .await

}