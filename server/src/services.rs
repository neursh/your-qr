use tokio::sync::{ mpsc, oneshot };

use crate::services::{ self, verify_pass::VerifyPassRequest };

pub mod hash_pass;
pub mod structs;
mod verify_pass;

pub fn construct_services(specs: WorkerSpecs) -> ServicesRequest {
  let (hash_pass_tx, hash_pass_rx) = mpsc::channel::<(String, oneshot::Sender<Option<String>>)>(
    specs.hash_pass.1
  );
  services::hash_pass::launch(hash_pass_rx, specs.hash_pass.0);

  let (verify_pass_tx, verify_pass_rx) = mpsc::channel::<
    (VerifyPassRequest, oneshot::Sender<Option<bool>>)
  >(specs.verify_pass.1);
  services::verify_pass::launch(verify_pass_rx, specs.verify_pass.0);

  ServicesRequest {
    hash_pass: RequestHandler { tx: hash_pass_tx },
    verify_pass: RequestHandler { tx: verify_pass_tx },
  }
}

// Tuple (thread amount, channel's buffer)
pub struct WorkerSpecs {
  pub hash_pass: (usize, usize),
  pub verify_pass: (usize, usize),
}

#[derive(Clone)]
pub struct ServicesRequest {
  pub hash_pass: RequestHandler<String, Option<String>>,
  pub verify_pass: RequestHandler<VerifyPassRequest, Option<bool>>,
}

#[derive(Clone)]
pub struct RequestHandler<R, P> {
  tx: mpsc::Sender<(R, oneshot::Sender<P>)>,
}

impl<R, P> RequestHandler<R, P> {
  pub async fn send(&self, request: R) -> Result<P, ()> {
    let (one_tx, one_rx) = oneshot::channel::<P>();

    if let Err(_) = self.tx.send((request, one_tx)).await {
      return Err(());
    }

    if let Ok(result) = one_rx.await {
      return Ok(result);
    } else {
      return Err(());
    }
  }
}
