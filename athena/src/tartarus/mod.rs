pub mod admin;
pub mod agent;

use dioxus::prelude::*;
use tokio;

pub async fn launch(config: ServeConfig, app: fn() -> Element) {
    // Connect to dioxus' logging infrastructure
    dioxus::logger::initialize_default();

    // Connect to the IP and PORT env vars passed by the Dioxus CLI (or your dockerfile)
    let socket_addr = dioxus_cli_config::fullstack_address_or_localhost();

    // Build a custom axum router
    let router = axum::Router::new()
        .serve_dioxus_application(config, app)
        .into_make_service();

    // And launch it!
    let listener = tokio::net::TcpListener::bind(socket_addr).await.unwrap();
    axum::serve(listener, router).await.unwrap();
}
