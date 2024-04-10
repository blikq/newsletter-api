use actix_web::*;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct FormData {
    pub name: String,
    pub email: String,
}

pub fn main(form: web::Form<FormData>) -> impl Responder{
    
    HttpResponse::Ok().body(format!("{} {}", form.name, form.email))
}