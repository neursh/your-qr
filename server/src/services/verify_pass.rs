use std::sync::Arc;

use argon2::{ Argon2, PasswordHash, PasswordVerifier };
use tokio::sync::{ Mutex, mpsc, oneshot };

pub struct VerifyPassRequest {
  pub password: String,
  pub hash: String,
  pub response: oneshot::Sender<Option<bool>>,
}

pub fn launch(rx: mpsc::Receiver<VerifyPassRequest>, amount: usize) {
  let rx_wraped = Arc::new(Mutex::new(rx));

  for _ in 0..amount {
    let rx_branch = rx_wraped.clone();
    tokio::task::spawn_blocking(|| { worker(rx_branch) });
  }
}

fn worker(rx: Arc<Mutex<mpsc::Receiver<VerifyPassRequest>>>) {
  let argon2 = Argon2::default();
  loop {
    let retrieve = { rx.blocking_lock().blocking_recv() };
    if let Some(request) = retrieve {
      if let Ok(hash) = PasswordHash::new(&request.hash) {
        let _ = request.response.send(
          Some(argon2.verify_password(request.password.as_bytes(), &hash).is_ok())
        );
      } else {
        let _ = request.response.send(None);
      }
    } else {
      // Something went wrong on the server, so just go ahead and crash ;)
      break;
    }
  }
}
