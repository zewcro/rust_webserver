use actix_web::{HttpResponse, Responder};

pub async fn get_users() -> impl Responder {
  HttpResponse::Ok().json(vec!["User1", "User2"])
}

pub async fn create_user() -> impl Responder {
  HttpResponse::Ok().body("User created successfully")
}