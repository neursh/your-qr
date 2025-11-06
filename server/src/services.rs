use tokio::sync::mpsc;

use crate::services::{ self, hash_pass::HashPassRequest };

pub mod hash_pass;
pub mod structs;

// Tuple (thread amount, channel's buffer)
pub struct WorkerSpecs {
  pub hash_pass: (usize, usize),
}

#[derive(Clone)]
pub struct ServicesRequest {
  pub hash_pass: mpsc::Sender<HashPassRequest>,
}

pub fn construct_services(specs: WorkerSpecs) -> ServicesRequest {
  let (hash_pass_tx, hash_pass_rx): (
    mpsc::Sender<HashPassRequest>,
    mpsc::Receiver<HashPassRequest>,
  ) = mpsc::channel::<HashPassRequest>(specs.hash_pass.1);
  services::hash_pass::launch(hash_pass_rx, specs.hash_pass.0);

  ServicesRequest {
    hash_pass: hash_pass_tx,
  }
}
