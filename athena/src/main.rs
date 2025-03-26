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
    let host = use_synced_storage::<LocalStorage, String>("host".to_string(), || {
        String::from("http://localhost:8080")
    });
    let token = use_synced_storage::<LocalStorage, String>("token".to_string(), || String::new());

    let api = use_context_provider(|| Signal::new(Api::new(&host(), &token())));

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
