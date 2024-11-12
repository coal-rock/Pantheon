// use serde::{Deserialize, Serialize};
// use tokio::io::{AsyncReadExt, AsyncWriteExt};
// use tokio::net::{TcpListener, TcpStream};
//
// #[derive(Serialize, Deserialize)]
// pub struct Request {
//     pub agent_id: String,
//     pub action: String,
//     pub payload: Option<String>,
// }
//
// #[derive(Serialize, Deserialize)]
// pub struct Response {
//     pub status: String,
//     pub message: Option<String>,
// }
//
// async fn handle_connection(mut socket: TcpStream) {
//     let mut buffer = vec![0; 1024];
//     if let Ok(n) = socket.read(&mut buffer).await {
//         let request: Request = serde_json::from_slice(&buffer[..n]).unwrap();
//         let response = process_request(request);
//
//         let response_data = serde_json::to_string(&response).unwrap();
//         let _ = socket.write_all(response_data.as_bytes()).await;
//     }
// }
//
// pub async fn start_server(addr: &str) -> Result<(), Box<dyn std::error::Error>> {
//     let listener = TcpListener::bind(addr).await?;
//     println!("Server listening on {}", addr);
//
//     loop {
//         let (socket, _) = listener.accept().await?;
//         tokio::spawn(handle_connection(socket));
//     }
// }
//
// fn process_request(request: Request) -> Response {
//     match request.action.as_str() {
//         "heartbeat" => Response {
//             status: "ok".to_string(),
//             message: Some("Heartbeat received".to_string()),
//         },
//         _ => Response {
//             status: "error".to_string(),
//             message: Some("Unknown action".to_string()),
//         },
//     }
// }
