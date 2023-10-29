use tokio::net::TcpStream;
use tokio::io::{AsyncWriteExt, AsyncReadExt};

#[tokio::main]
async fn main() {
  let mut stream = TcpStream::connect("127.0.0.1:8084").await.unwrap();
  let mut buf = [0; 1024];
  let n = stream.read(&mut buf).await.unwrap();
  let challenge = String::from_utf8_lossy(&buf[..n]).to_string();
  let nonce = solve_challenge(&challenge);  // Assume this function is implemented
  stream.write_all(nonce.as_bytes()).await.unwrap();
  let n = stream.read(&mut buf).await.unwrap();
  let quote = String::from_utf8_lossy(&buf[..n]).to_string();
  println!("{}", quote);
}

fn solve_challenge(_challenge: &str) -> String {
  // Placeholder, implement PoW solving algorithm
  "some-nonce".to_string()
}
