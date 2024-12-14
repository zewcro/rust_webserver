use serde::{Serializer, Deserialize}; 

#[derive(Serialize, Deserialize)]
pub struct User {
  pub id i32,
  pub name: String,
  pub email: String,
}