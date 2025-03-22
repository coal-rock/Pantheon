use dioxus::prelude::*;

pub mod components;
pub mod services;
pub mod views;

use services::api::Api;
use views::home::Home;

#[derive(Debug, Clone, Routable, PartialEq)]
enum Route {
    #[route("/")]
    Home {},
}

#[component]
fn App() -> Element {
    let api = use_context_provider(|| Api::new("http://localhost:8080/"));

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
