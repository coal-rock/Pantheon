use dioxus::prelude::*;

#[cfg(feature = "server")]
pub mod tartarus;

pub mod components;
pub mod views;

use views::home::Home;

#[derive(Debug, Clone, Routable, PartialEq)]
enum Route {
    #[route("/")]
    Home {},
}

#[component]
fn App() -> Element {
    rsx! {
        document::Link { rel: "icon", href: asset!("/assets/favicon.ico") }
        document::Link { rel: "stylesheet", href: asset!("/assets/tailwind.css") }
        document::Link { rel: "stylesheet", href: asset!("/assets/style.css") }
        // script { src: asset!("/assets/draggable.min.js") }

        Router::<Route> {}
    }
}

fn main() {
    #[cfg(feature = "server")]
    tokio::runtime::Runtime::new()
        .unwrap()
        .block_on(tartarus::launch(ServeConfig::new().unwrap(), App));

    #[cfg(not(feature = "server"))]
    dioxus::launch(App);
}
