use dioxus::prelude::*;

use crate::components::navbar::Navbar;
use crate::components::sidebar::Sidebar;

#[component]
pub fn Home() -> Element {
    rsx! {
        div {
            class: "flex flex-col h-screen",

            Navbar { }
            div {
                class: "flex flex-row grow",

                Sidebar {}
                div {
                    class: "grow bg-zinc-900",
                }
            }
        }
    }
}
