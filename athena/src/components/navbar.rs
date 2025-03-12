use dioxus::prelude::*;

use crate::Route;

#[component]
pub fn Navbar() -> Element {
    rsx! {
        div {
            class: "bg-zinc-950 h-16 flex items-center justify-between",
            Link {
                class: "text-white font-sans text-4xl p-4",
                to: Route::Home {},
                "Athena"
            }
            h1 {
                class: "text-gray-400 font-sans text-md p-4",
                "v0.0.1"
            }
        }
        Outlet::<Route> {}
    }
}
