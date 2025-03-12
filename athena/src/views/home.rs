use dioxus::prelude::*;

use crate::components::navbar::Navbar;

#[component]
pub fn Home() -> Element {
    rsx! {
        div {
            class: "flex flex-col h-screen",
            Navbar { }
            div {
                class: "grow bg-zinc-900",
            }
        }
    }
}
