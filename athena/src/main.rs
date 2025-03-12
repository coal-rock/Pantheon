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
        Router::<Route> {}
    }
}
