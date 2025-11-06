use std::sync::Arc;

use argon2::{ Argon2, password_hash::{ PasswordHasher, SaltString, rand_core::OsRng } };
use tokio::sync::{ Mutex, mpsc, oneshot };

pub struct HashPassRequest {
  pub password: String,
  pub response: oneshot::Sender<Option<String>>,
}

pub fn launch(rx: mpsc::Receiver<HashPassRequest>, amount: usize) {
  let rx_wraped = Arc::new(Mutex::new(rx));

  for _ in 0..amount {
    let rx_branch = rx_wraped.clone();
    tokio::task::spawn_blocking(|| { worker(rx_branch) });
  }
}

fn worker(rx: Arc<Mutex<mpsc::Receiver<HashPassRequest>>>) {
  let argon2 = Argon2::default();
  loop {
    let retrieve = { rx.blocking_lock().blocking_recv() };
    if let Some(request) = retrieve {
      let salt = SaltString::generate(&mut OsRng);
      match argon2.hash_password(request.password.as_bytes(), &salt) {
        Ok(hashed) => {
          let _ = request.response.send(Some(hashed.to_string()));
        }
        Err(_) => {
          let _ = request.response.send(None);
        }
      }
    } else {
      // Something went wrong on the server, so just go ahead and crash ;)
      break;
    }
  }
}
