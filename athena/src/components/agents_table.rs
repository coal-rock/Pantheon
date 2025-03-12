use dioxus::prelude::*;

use dioxus_free_icons::icons::fa_solid_icons::FaMagnifyingGlass;
use dioxus_free_icons::Icon;

#[component]
pub fn AgentsTable() -> Element {
    rsx! {
        div {
            class: "bg-zinc-950 w-full h-full rounded flex flex-col p-4 drop-shadow-sm draggable",
            div {
                class: "text-gray-300 text-xl font-sans pl-1 handle cursor-grab active:cursor-grab",
                "Agents Table"
            }
            div {
                class: "flex items-center h-4",
                hr {
                    class: "w-full",
                }
            }
            div {
                class: "bg-zinc-900 w-full rounded-xs border-b-1 border-gray-400 h-8 flex items-center pl-2",
                Icon {
                    width: 16,
                    icon: FaMagnifyingGlass,
                    fill: "lightgray"
                }
                input {
                    class: "w-full h-full text-gray-300 outline-none pl-2",
                    value: "",
                    placeholder: "agent name",
                }
            }
        }
    }
}
