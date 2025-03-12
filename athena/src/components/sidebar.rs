use dioxus::prelude::*;

use crate::Route;

#[component]
pub fn Sidebar() -> Element {
    rsx! {
        div {
            class: "grow-0 w-72 bg-zinc-950 border-r-2 flex flex-col",

            SidebarElement{
                text: "Agents",
                to: Route::Home {},
            }

            SidebarElement{
                text: "Settings",
                to: Route::Home {},
            }

            SidebarElement{
                text: "About",
                to: Route::Home {},
            }

            SidebarElement{
                text: "Downloads",
                to: Route::Home {},
            }

            SidebarElement{
                text: "Alerts",
                to: Route::Home {},
            }
        }
    }
}

#[component]
fn SidebarElement(text: String, to: Route) -> Element {
    rsx! {
        Link {
            class: "border-zinc-800 hover:bg-zinc-900 border-b-1 flex justify-start items-center h-14 hover:underline decoration-2 underline-offset-4 decoration-blue-500",
            to: "{to}",
            div {
                class: "text-white pl-2",
                "{text}"
            }
        }
    }
}
