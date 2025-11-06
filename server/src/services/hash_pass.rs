use std::sync::Arc;

use argon2::{ Argon2, password_hash::{ PasswordHasher, SaltString, rand_core::OsRng } };
use tokio::sync::{ Mutex, mpsc, oneshot };

pub fn launch(rx: mpsc::Receiver<(String, oneshot::Sender<Option<String>>)>, amount: usize) {
  let rx_wraped = Arc::new(Mutex::new(rx));

  for _ in 0..amount {
    let rx_branch = rx_wraped.clone();
    tokio::task::spawn_blocking(|| { worker(rx_branch) });
  }
}

fn worker(rx: Arc<Mutex<mpsc::Receiver<(String, oneshot::Sender<Option<String>>)>>>) {
  let argon2 = Argon2::default();
  loop {
    let retrieve = { rx.blocking_lock().blocking_recv() };
    if let Some(request) = retrieve {
      let salt = SaltString::generate(&mut OsRng);
      match argon2.hash_password(request.0.as_bytes(), &salt) {
        Ok(hashed) => {
          let _ = request.1.send(Some(hashed.to_string()));
        }
        Err(_) => {
          let _ = request.1.send(None);
        }
      }
    } else {
      // Something went wrong on the server, so just go ahead and crash ;)
      break;
    }
  }
}
