use log::{error, info};
use simple_logger::SimpleLogger;
use std::error::Error;

use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;

mod utils {
  pub mod sha2_256;
}
use utils::sha2_256::solve_challenge;


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
    let challenge = read_string_from_stream(&mut stream).await?;
    info!("Received challenge: {}", challenge);

    let nonce = solve_challenge(&challenge)?;
    stream.write_all(nonce.as_bytes()).await?;

    let quote = read_string_from_stream(&mut stream).await?;
    info!("Received quote: {}", quote);
    Ok(())
}

async fn read_string_from_stream(stream: &mut TcpStream) -> Result<String, Box<dyn Error>> {
    let mut buf = [0; 1024];
    let n = stream.read(&mut buf).await?;
    Ok(String::from_utf8_lossy(&buf[..n]).to_string())
}

