use std::sync::Arc;

use axum::{ Router, routing::post };
use tokio::sync::Semaphore;

use crate::{ database::KewarCollections, services::Services };

pub mod create;
pub mod delete;
pub mod structs;

#[derive(Clone)]
pub struct RoutesLimiter {
  pub create: Arc<Semaphore>,
}

#[derive(Clone)]
pub struct RouteState {
  pub services: Services,
  pub kewar_collections: KewarCollections,
  pub limiters: RoutesLimiter,
}

pub fn routes(services: Services, kewar_collections: KewarCollections) -> Router {
  let state = RouteState {
    services,
    kewar_collections,
    limiters: RoutesLimiter { create: Arc::new(Semaphore::new(8)) },
  };
  Router::new().route("/", post(create::handle).delete(delete::handle)).with_state(state)
}
