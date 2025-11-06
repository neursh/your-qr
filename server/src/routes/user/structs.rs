use serde::Deserialize;

#[derive(Deserialize)]
pub struct CreateUserPayload {
  pub email: String,
  pub password: String,
  pub name: Option<String>,
}
