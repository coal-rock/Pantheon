use tokio::net::TcpStream;
use tokio::io::{AsyncWriteExt, AsyncReadExt};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Request {
    pub agent_id: String,
    pub action: String,
    pub payload: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct Response {
    pub status: String,
    pub message: Option<String>,
}

pub async fn send_request(server_addr: &str, request: Request) -> Result<Response, Box<dyn std::error::Error>> {
    let mut stream = TcpStream::connect(server_addr).await?;
    let request_data = serde_json::to_string(&request)?;

    stream.write_all(request_data.as_bytes()).await?;
    let mut response_data = vec![0; 1024];
    let n = stream.read(&mut response_data).await?;
    let response: Response = serde_json::from_slice(&response_data[..n])?;

    Ok(response)
}
