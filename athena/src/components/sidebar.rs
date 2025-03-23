use dioxus::prelude::*;

use crate::Route;

use dioxus_free_icons::icons::fa_solid_icons::{FaBell, FaDownload, FaGear, FaInfo, FaRobot};
use dioxus_free_icons::Icon;

#[component]
pub fn Sidebar(should_show: Signal<bool>) -> Element {
    // TODO: make icons slightly change color on hover
    rsx! {
        div {
            class: "grow-0 w-64 bg-zinc-950 border-r-2 border-gray-600 flex flex-col",
            display: if *should_show.read() { None } else { Some("none") },

            SidebarElement{
                text: "Agents",
                to: Route::Home {},
                icon: rsx!(Icon { icon: FaRobot })
            }

            SidebarElement{
                text: "Settings",
                to: Route::Home {},
                icon: rsx!(Icon { icon: FaGear })
            }

            SidebarElement{
                text: "About",
                to: Route::Home {},
                icon: rsx!(Icon { icon: FaInfo })
            }

            SidebarElement{
                text: "Downloads",
                to: Route::Home {},
                icon: rsx!(Icon { icon: FaDownload })
            }

            SidebarElement{
                text: "Alerts",
                to: Route::Home {},
                icon: rsx!(Icon { icon: FaBell })
            }
        }
    }
}

#[component]
fn SidebarElement(text: String, to: Route, icon: Element) -> Element {
    rsx! {
        Link {
            class: "border-zinc-800 hover:bg-zinc-900 border-b-1 flex justify-start items-center h-14 hover:underline decoration-2 underline-offset-4 decoration-blue-500",
            to: "{to}",
            div {
                class: "text-white pl-4 flex stroke-current",
                {icon}
                div {
                    class: "pl-4",
                    "{text}"
                }
            }
        }
    }
}
