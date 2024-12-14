use actix_web::web;
use crate::handlers::{user_handler, health_handler};

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg
        .service(web::scope("/users")
            .route("/", web::get().to(user_handler::get_users))
            .route("/", web::post().to(user_handler::create_user)))
        .service(web::scope("/health")
            .route("/", web::get().to(health_handler::check_health)));
}