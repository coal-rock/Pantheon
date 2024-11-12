use serde::{Deserialize, Serialize};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;

// pub async fn send_instruction() {}
// pub async fn
// pub async fn send_request(
//     server_addr: &str,
//     request: Request,
// ) -> Result<Response, Box<dyn std::error::Error>> {
//     let mut stream = TcpStream::connect(server_addr).await?;
//     let request_data = serde_json::to_string(&request)?;
//
//     stream.write_all(request_data.as_bytes()).await?;
//     let mut response_data = vec![0; 1024];
//     let n = stream.read(&mut response_data).await?;
//     let response: Response = serde_json::from_slice(&response_data[..n])?;
//
//     Ok(response)
// }
