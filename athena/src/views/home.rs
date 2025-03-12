use dioxus::prelude::*;

use crate::components::navbar::Navbar;
use crate::components::sidebar::Sidebar;

#[component]
pub fn Home() -> Element {
    let show_sidebar = use_signal(|| true);

    rsx! {
        div {
            class: "flex flex-col h-screen",

            Navbar {
                show_sidebar: show_sidebar,
            }
            div {
                class: "flex flex-row grow",

                Sidebar {
                    should_show: show_sidebar
                }
                div {
                    class: "grow bg-zinc-900",
                }
            }
        }
    }
}
