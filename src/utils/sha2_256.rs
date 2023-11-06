use log::info;
use std::error::Error;
use sha2::Digest;
use std::time::Instant;

pub fn solve_challenge(challenge: &str) -> Result<String, Box<dyn Error>> {
  let parts: Vec<&str> = challenge.split(':').collect();
  let random_string = parts.get(0).ok_or("Invalid challenge format")?;
  let required_zeros = parts.get(1).ok_or("Invalid challenge format")?.as_bytes();
  let mut nonce = 0;

  let start_time = Instant::now();

  loop {
      let attempt = format!("{}{}", random_string, nonce);
      let hash = sha2::Sha256::digest(attempt.as_bytes());

      if nonce % 100_000 == 0 {
          info!("Current nonce: {}, Hash: {:?}", nonce, hash);
      }
      if hash.starts_with(required_zeros) {
          let duration = start_time.elapsed();
          info!("Nonce: {}, Time: {:?}", nonce, duration);
          return Ok(nonce.to_string());
      }
      nonce += 1;
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_solve_challenge() {
      let challenge = "random_string:0";
      match solve_challenge(&challenge) {
          Ok(nonce) => assert!(nonce.len() > 0),
          Err(e) => panic!("Ошибка: {:?}", e),
      }
  }
}
