use sqlx::{PgPool, Pool, Postgres}; 

pub async fn establic_connection(database_url: &str) -> PgPool {
  PgPool::connect(database_url)
  .await 
  .expect("Failed to connect to the database")
}