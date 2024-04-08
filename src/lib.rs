use actix_web::*;
use std::sync::Mutex;


#[get("/health_check")]
async fn health_check() -> impl Responder {
    HttpResponse::Ok()
}

pub async fn run() -> std::io::Result<()> {
    
    HttpServer::new(move || App::new()
    .service(health_check)
    )
    .bind("127.0.0.1:8080")?
    .run()
    .await

}