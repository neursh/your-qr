use axum::{ Router, routing::post };

use crate::{ database::KewarCollections, services::Services };

pub mod create;
pub mod delete;
pub mod structs;

#[derive(Clone)]
pub struct RouteState {
  pub services: Services,
  pub kewar_collections: KewarCollections,
}

pub fn routes(services: Services, kewar_collections: KewarCollections) -> Router {
  let state = RouteState {
    services,
    kewar_collections,
  };
  Router::new().route("/", post(create::handle).delete(delete::handle)).with_state(state)
}
