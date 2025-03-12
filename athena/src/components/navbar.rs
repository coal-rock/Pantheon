use dioxus::prelude::*;

use crate::Route;

#[component]
pub fn Navbar() -> Element {
    rsx! {
        div {
            class: "bg-zinc-950 h-16 flex items-center justify-between border-b-2",
            div {
                class: "flex flex-col p-2",
                Link {
                    class: "text-white font-sans text-4xl",
                    to: Route::Home {},
                    "Athena"
                }
                h1 {
                    class: "text-gray-400 font-sans text-sm",
                    "v0.0.1"
                }
            }
            a {
                href: "https://github.com/Dack985/Pantheon",
                img {
                    class: "h-10 pr-2",
                    src: asset!("assets/github-logo.svg"),
                }
            }
        }
    }
}
