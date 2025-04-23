use dioxus::prelude::*;

use crate::components::navbar::Navbar;
use crate::components::sidebar::Sidebar;

#[component]
pub fn SwaplessPage(children: Element) -> Element {
    let show_sidebar = use_signal(|| true);

    rsx! {
        div {
            class: "flex flex-col h-screen",

            Navbar {
                anemic: false,
            }
            div {
                class: "flex flex-row grow",

                Sidebar {
                    should_show: show_sidebar
                }

                {children}
            }
        }
    }
}
