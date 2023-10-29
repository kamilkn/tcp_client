use tokio::net::TcpStream;
use tokio::io::{AsyncWriteExt, AsyncReadExt};
use sha2::Digest;
use std::time::{Instant};

#[tokio::main]
async fn main() {
  let mut stream = TcpStream::connect("127.0.0.1:8084").await.unwrap();
  let mut buf = [0; 1024];
  let n = stream.read(&mut buf).await.unwrap();
  let challenge = String::from_utf8_lossy(&buf[..n]).to_string();
  println!("Received challenge: {}", challenge);
  let nonce = solve_challenge(&challenge);
  stream.write_all(nonce.as_bytes()).await.unwrap();
  let n = stream.read(&mut buf).await.unwrap();
  let quote = String::from_utf8_lossy(&buf[..n]).to_string();
  println!("Received quote: {}", quote);
}

fn solve_challenge(challenge: &str) -> String {
  // Assume the challenge format is "random_string:00000"
  let parts: Vec<&str> = challenge.split(':').collect();
  let random_string = parts[0];
  let mut nonce = 0;

  let start_time = Instant::now();

  loop {
      let attempt = format!("{}{}", random_string, nonce);
      let hash = sha2::Sha256::digest(attempt.as_bytes());
      if hash.starts_with(b"000") {  // Assuming the requirement is 5 leading zeros
          let duration = start_time.elapsed();
          println!("Затраченное время: {:?}", duration); 
          return nonce.to_string();
      }
      if nonce % 10_000 == 0 {  // Логирование каждые 10,000 итераций
          println!("Current nonce: {}, Hash: {:?}", nonce, hash);
      }
      nonce += 1;
  }
}
