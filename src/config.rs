use serde::Deserialize;
use dotenv::dotenv;
use std::env;

#[derive(Deserialize)]
pub struct Config {
    pub host: String,
    pub port: u16,  
    pub database_url: String,
  }



pub fn get_config() -> Config {
    dotenv().ok(); 

    Config {
      host: env::var("HOST").unwrap_or_else(|_| "127.0.0.1".to_string()),
      port: env::var("PORT").unwrap_or_else(|_| "8080".to_string()).parse().unwrap(),
      database_url: env::var("DATABASE_URL").expect("DATABASE_URL must be set"),

    }
}