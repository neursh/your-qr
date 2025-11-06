use serde::{ Deserialize, Serialize };

#[derive(Deserialize, Serialize)]
pub struct HashedPassword {
  pub hashed: String,
  pub salt: String,
}
