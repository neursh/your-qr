use axum::{ Json, extract::State, http::StatusCode };

use crate::{ routes::users::structs::CreateUserPayload, services::ServicesRequest };

pub async fn handle(
  State(services): State<ServicesRequest>,
  Json(payload): Json<CreateUserPayload>
) -> Result<(StatusCode, String), StatusCode> {
  if let Ok(Some(hashed)) = services.hash_pass.send(payload.password).await {
    return Ok((StatusCode::OK, hashed));
  } else {
    return Err(StatusCode::INTERNAL_SERVER_ERROR);
  }
}
