use actix_web::{web, App, HttpServer}; 
mod config; 
mod handlers; 
mod routes; 
mod db;


#[actix_web::main]
async fn main() -> std::io::Result<()>{
  let config = config::get_config(); 
  env_logger::init();

  HttpServer::new(|| {
    App::new()
        .configure(routes::configure_routes)
  })
  .bind((config.host.as_str(), config_port))?
  .run()
  .await
}