use axum::{ Router, routing::post };

use crate::services::ServicesRequestChannel;

pub mod create;
pub mod structs;

pub fn routes(services_channel: ServicesRequestChannel) -> Router {
  Router::new().route("/", post(create::handle)).with_state(services_channel)
}
