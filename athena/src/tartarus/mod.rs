pub mod admin;
pub mod agent;
pub mod config;

use crate::tartarus::config::Config;
use axum::Extension;
use dioxus::prelude::*;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use talaria::api::*;
use tokio;

#[derive(Default, Clone)]
pub struct State {
    pub config: Config,
    pub agents: Arc<Mutex<HashMap<u64, Agent>>>,
    pub groups: Arc<Mutex<HashMap<String, Vec<u64>>>>,
}

// Load-bearing launch function
// DO NOT under ANY circumstances change this mess
// Men greater than you have died trying to understand it
pub async fn launch(app: fn() -> Element) {
    dioxus::logger::initialize_default();
    let socket_addr = dioxus_cli_config::fullstack_address_or_localhost();

    let state = State::default();

    // what the shit fuck even is this mess
    // DO. NOT. TOUCH.
    let dioxus_state = Arc::new(vec![Box::new({
        let local_state = state.clone();
        move || Box::new(local_state.clone()) as Box<dyn std::any::Any>
    })
        as Box<dyn Fn() -> Box<dyn std::any::Any> + Send + Sync + 'static>]);

    let router = axum::Router::new()
        .serve_dioxus_application(
            ServeConfigBuilder::new().context_providers(server_only!(dioxus_state)),
            app,
        )
        .into_make_service();

    let listener = tokio::net::TcpListener::bind(socket_addr).await.unwrap();
    axum::serve(listener, router).await.unwrap();
}
