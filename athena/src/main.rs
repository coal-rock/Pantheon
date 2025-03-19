use dioxus::logger::tracing::Level;
use dioxus::prelude::*;

pub mod components;
pub mod views;

use views::home::Home;
use views::page::Page;

#[derive(Debug, Clone, Routable, PartialEq)]
enum Route {
    #[route("/")]
    Home {},
}

fn main() {
    dioxus::logger::init(Level::INFO).expect("logger failed to init");
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Link { rel: "icon", href: asset!("/assets/favicon.ico") }
        document::Link { rel: "stylesheet", href: asset!("/assets/tailwind.css") }
        document::Link { rel: "stylesheet", href: asset!("/assets/style.css") }
        script { src: asset!("/assets/draggable.min.js") }

        Router::<Route> {}
    }
}
