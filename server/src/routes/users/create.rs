use axum::{ Json, extract::State, http::StatusCode };
use tokio::sync::oneshot;

use crate::{
  routes::users::structs::CreateUserPayload,
  services::{ ServicesRequestChannel, hash_pass::HashPassRequest, structs::HashedPassword },
};

pub async fn handle(
  State(services): State<ServicesRequestChannel>,
  Json(payload): Json<CreateUserPayload>
) -> Result<(StatusCode, Json<HashedPassword>), StatusCode> {
  let (hash_tx, hash_rx) = oneshot::channel::<Option<HashedPassword>>();

  if
    let Err(_) = services.hash_pass.send(HashPassRequest {
      password: payload.password,
      response: hash_tx,
    }).await
  {
    return Err(StatusCode::INTERNAL_SERVER_ERROR);
  }

  if let Ok(Some(hashed)) = hash_rx.await {
    return Ok((StatusCode::OK, Json(hashed)));
  } else {
    return Err(StatusCode::INTERNAL_SERVER_ERROR);
  }
}
