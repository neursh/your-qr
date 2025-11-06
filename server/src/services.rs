use tokio::sync::mpsc;

use crate::services::{ self, hash_pass::HashPassRequest, verify_pass::VerifyPassRequest };

pub mod hash_pass;
pub mod structs;
mod verify_pass;

// Tuple (thread amount, channel's buffer)
pub struct WorkerSpecs {
  pub hash_pass: (usize, usize),
  pub verify_pass: (usize, usize),
}

#[derive(Clone)]
pub struct ServicesRequest {
  pub hash_pass: mpsc::Sender<HashPassRequest>,
  pub verify_pass: mpsc::Sender<VerifyPassRequest>,
}

pub fn construct_services(specs: WorkerSpecs) -> ServicesRequest {
  let (hash_pass_tx, hash_pass_rx) = mpsc::channel::<HashPassRequest>(specs.hash_pass.1);
  services::hash_pass::launch(hash_pass_rx, specs.hash_pass.0);

  let (verify_pass_tx, verify_pass_rx) = mpsc::channel::<VerifyPassRequest>(specs.verify_pass.1);
  services::verify_pass::launch(verify_pass_rx, specs.verify_pass.0);

  ServicesRequest {
    hash_pass: hash_pass_tx,
    verify_pass: verify_pass_tx,
  }
}
