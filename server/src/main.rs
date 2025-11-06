use std::env;
use axum::Router;

use crate::services::{ WorkerSpecs, construct_services };

mod routes;
mod services;

#[tokio::main]
async fn main() {
  let args: Vec<String> = env::args().collect();
  let host = &args[1];

  let services = construct_services(WorkerSpecs {
    // Allocate a reasonable amount of workers for password services.
    // This be using 100% when full load on all workers.
    // Password hashing is heavy after all.
    hash_pass: (4, 2048),
    verify_pass: (4, 2048),
  });

  let app = Router::new().nest("/users", routes::users::routes(services));

  let listener = tokio::net::TcpListener::bind(host).await.unwrap();
  axum::serve(listener, app).await.unwrap();
}
