use dioxus::prelude::*;

pub mod components;
pub mod services;
pub mod views;

use dioxus_sdk::storage::{use_synced_storage, LocalStorage};
use services::api::Api;
use views::{authenticate::Authenticate, home::Home};

#[derive(Debug, Clone, Routable, PartialEq)]
enum Route {
    #[route("/")]
    Home {},
    #[route("/authenticate")]
    Authenticate {},
}

#[component]
fn App() -> Element {
    let mut api = use_context_provider(|| Api::new("http://localhost:8080/"));

    let host = use_synced_storage::<LocalStorage, String>("host".to_string(), || String::new());
    let token = use_synced_storage::<LocalStorage, String>("token".to_string(), || String::new());

    api.set_token(&token());
    api.set_api_base(&host());

    rsx! {
        document::Link { rel: "icon", href: asset!("/assets/favicon.ico") }
        document::Link { rel: "stylesheet", href: asset!("/assets/tailwind.css") }
        document::Link { rel: "stylesheet", href: asset!("/assets/style.css") }
        // script { src: asset!("/assets/draggable.min.js") }

        Router::<Route> {}
    }
}

fn main() {
    dioxus::launch(App);
}
