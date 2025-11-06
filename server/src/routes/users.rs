use axum::{ Router, routing::post };

use crate::services::ServicesRequest;

pub mod create;
pub mod structs;

pub fn routes(services: ServicesRequest) -> Router {
  Router::new().route("/", post(create::handle)).with_state(services)
}
