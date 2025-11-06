use std::env;
use axum::Router;

use crate::services::{ WorkerSpecs, construct_services };

mod routes;
mod services;

#[tokio::main]
async fn main() {
  let args: Vec<String> = env::args().collect();
  let host = &args[1];

  let services_channel = construct_services(WorkerSpecs {
    hash_pass: (4, 2048),
  });

  let app = Router::new().nest("/users", routes::users::routes(services_channel));

  let listener = tokio::net::TcpListener::bind(host).await.unwrap();
  axum::serve(listener, app).await.unwrap();
}
