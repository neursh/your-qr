use axum::{ Router, routing::post };

use crate::services::Services;

pub mod create;
pub mod structs;

pub fn routes(services: Services) -> Router {
  Router::new().route("/", post(create::handle)).with_state(services)
}
