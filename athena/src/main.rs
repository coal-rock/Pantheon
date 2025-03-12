use dioxus::prelude::*;

pub mod components;
pub mod views;

use views::home::Home;

#[derive(Debug, Clone, Routable, PartialEq)]
enum Route {
    #[route("/")]
    Home {},
}

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: asset!("/assets/tailwind.css") }
        script { src: "https://cdn.jsdelivr.net/npm/@shopify/draggable/build/umd/index.min.js" }
        Router::<Route> {}
    }
}
