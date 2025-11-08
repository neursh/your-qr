use axum::{ Json, extract::State, http::StatusCode };

use crate::{
  base::{ self, response::ResponseModel },
  database::users::UserStore,
  routes::user::{ RouteState, structs::CreateUserPayload },
};
use nanoid::nanoid;

pub async fn handle(
  State(state): State<RouteState>,
  Json(payload): Json<CreateUserPayload>
) -> (StatusCode, Json<ResponseModel>) {
  let password_hash = match state.services.hash_pass.send(payload.password).await {
    Ok(Some(hash)) => hash,
    _ => {
      return base::response::error(
        StatusCode::INTERNAL_SERVER_ERROR,
        "Something went wrong while processing your request."
      );
    }
  };

  let user = UserStore {
    _id: nanoid!(),
    email: payload.email,
    password_hash,
  };

  match state.kewar_collections.users.add(user).await {
    Ok(_) => base::response::success(StatusCode::CREATED),
    Err(error) => parse_db_fail(error),
  }
}

fn parse_db_fail(error: mongodb::error::Error) -> (StatusCode, Json<ResponseModel>) {
  use mongodb::error::{ ErrorKind, WriteFailure };

  match *error.kind {
    ErrorKind::Write(WriteFailure::WriteError(ref write_error)) if write_error.code == 11000 => {
      base::response::error(
        StatusCode::BAD_REQUEST,
        "An account with the same email already exists."
      )
    }
    _ => {
      eprintln!("Database error: {:?}", error.kind);
      return base::response::error(
        StatusCode::INTERNAL_SERVER_ERROR,
        "Something went wrong while processing your request."
      );
    }
  }
}
