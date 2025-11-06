use tokio::sync::{ mpsc::{ self, Receiver }, oneshot };

use crate::services::verify_pass::VerifyPassRequest;

pub mod hash_pass;
pub mod structs;
pub mod verify_pass;

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

pub fn construct_services(specs: WorkerSpecs) -> ServicesRequest {
  ServicesRequest {
    hash_pass: RequestHandler::new(hash_pass::launch, specs.hash_pass.0, specs.hash_pass.1),
    verify_pass: RequestHandler::new(verify_pass::launch, specs.verify_pass.0, specs.verify_pass.1),
  }
}

#[derive(Clone)]
pub struct RequestHandler<R, P> {
  tx: mpsc::Sender<(R, oneshot::Sender<P>)>,
}
impl<R, P> RequestHandler<R, P> {
  pub fn new<F: Fn(Receiver<(R, oneshot::Sender<P>)>, usize)>(
    launcher: F,
    amount: usize,
    buffer: usize
  ) -> Self {
    let (service_tx, service_rx) = mpsc::channel::<(R, oneshot::Sender<P>)>(buffer);
    launcher(service_rx, amount);

    RequestHandler {
      tx: service_tx,
    }
  }

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
