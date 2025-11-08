use std::env;
use axum::Router;

use crate::services::{ Services, WorkerSpecs };

mod routes;
mod services;
mod database;
mod base;

#[tokio::main]
async fn main() {
  dotenv::dotenv().ok();

  let mongodb_connection_string = std::env
    ::var("MONGODB_CONNECTION_STRING")
    .expect("MONGODB_CONNECTION_STRING must be set in .env file");

  let args: Vec<String> = env::args().collect();
  let host = &args[1];

  let kewar_collections = database::initialize(&mongodb_connection_string).await.unwrap();

  let services = Services::new(WorkerSpecs {
    // Allocate a reasonable amount of workers for password services.
    // This be using 100% when full load on all workers.
    // Password hashing is heavy after all.
    hash_pass: (8, 2048),
    verify_pass: (8, 2048),
  });

  let app = Router::new().nest("/user", routes::user::routes(services, kewar_collections));

  let listener = tokio::net::TcpListener::bind(host).await.unwrap();
  axum::serve(listener, app).await.unwrap();
}
