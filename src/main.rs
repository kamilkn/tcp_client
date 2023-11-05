use log::{error, info};
use sha2::Digest;
use simple_logger::SimpleLogger;
use std::error::Error;
use std::time::Instant;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;

#[tokio::main]
async fn main() {
    SimpleLogger::new().init().unwrap();

    match run().await {
        Ok(()) => info!("Program completed successfully"),
        Err(e) => error!("Error: {}", e),
    }
}

async fn run() -> Result<(), Box<dyn Error>> {
    let mut stream = TcpStream::connect("127.0.0.1:8084").await?;
    let mut buf = [0; 1024];
    let n = stream.read(&mut buf).await?;
    let challenge = String::from_utf8_lossy(&buf[..n]).to_string();
    info!("Received challenge: {}", challenge);
    let nonce = solve_challenge(&challenge)?;

    stream.write_all(nonce.as_bytes()).await?;
    let n = stream.read(&mut buf).await?;
    let quote = String::from_utf8_lossy(&buf[..n]).to_string();
    info!("Received quote: {}", quote);
    Ok(())
}



fn solve_challenge(challenge: &str) -> Result<String, Box<dyn Error>> {
    let parts: Vec<&str> = challenge.split(':').collect();
    let random_string = parts.get(0).ok_or("Invalid challenge format")?;
    let required_zeros = parts.get(1).ok_or("Invalid challenge format")?.as_bytes();
    let mut nonce = 0;

    let start_time = Instant::now();

    loop {
        let attempt = format!("{}{}", random_string, nonce);
        let hash = sha2::Sha256::digest(attempt.as_bytes());
        if hash.starts_with(required_zeros) {
            let duration = start_time.elapsed();
            info!("Time: {:?}", duration);
            return Ok(nonce.to_string());
        }
        if nonce % 100_000 == 0 {
            info!("Current nonce: {}, Hash: {:?}", nonce, hash);
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
            Ok(nonce) => assert!(nonce.len() > 0),  // Проверьте, что nonce не пустой
            Err(e) => panic!("Ошибка: {:?}", e),  // Паника в случае ошибки
        }
    }
}

