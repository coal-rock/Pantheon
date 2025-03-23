use dioxus::prelude::*;

use dioxus_free_icons::icons::fa_solid_icons::FaMagnifyingGlass;
use dioxus_free_icons::Icon;

use crate::components::panel_base::PanelBase;
use crate::views::page::PanelManager;

#[component]
pub fn Placeholder() -> Element {
    rsx! {
        div {
            class: "bg-transparent w-full h-full rounded-xs flex flex-col p-4 draggable border-2 border-gray-500",
        }
    }
}
