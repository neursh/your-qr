use axum::{ Json, extract::State, http::StatusCode };

use crate::{ routes::user::structs::CreateUserPayload, services::Services };

pub async fn handle(
  State(services): State<Services>,
  Json(payload): Json<CreateUserPayload>
) -> Result<(StatusCode, String), StatusCode> {
  if let Ok(Some(hashed)) = services.hash_pass.send(payload.password).await {
    return Ok((StatusCode::OK, hashed));
  } else {
    return Err(StatusCode::INTERNAL_SERVER_ERROR);
  }
}
