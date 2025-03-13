use dioxus::prelude::*;

use dioxus_free_icons::icons::fa_solid_icons::FaMagnifyingGlass;
use dioxus_free_icons::Icon;

#[component]
pub fn AgentsTable() -> Element {
    let show_windows = use_signal(|| true);
    let show_linux = use_signal(|| true);
    let show_inactive = use_signal(|| true);

    rsx! {
        div {
            class: "bg-zinc-950 w-full h-full rounded-xs flex flex-col p-4 drop-shadow draggable border-2 border-gray-500",
            div {
                class: "text-gray-300 text-xl font-sans pl-1 handle cursor-grab active:cursor-grab",
                "Agent Table"
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
            div {
                class: "flex flex-row gap-2 pt-2",
                Checkbox{
                    id: "show-windows",
                    text: "Show Windows",
                    checked: show_windows,
                }
                "|"
                Checkbox{
                    id: "show-linux",
                    text: "Show Linux",
                    checked: show_linux,
                }
                "|"
                Checkbox{
                    id: "show-inactive",
                    text: "Show Inactive",
                    checked: show_inactive,
                }
            }
        }
    }
}

#[component]
fn Checkbox(text: String, id: String, checked: Signal<bool>) -> Element {
    // Flowbite
    rsx! {
        div {
            class: "flex items-center",
            input {
                class: "w-4 h-4 text-blue-600 focus:ring-blue-600 ring-offset-gray-800 focus:none bg-gray-700 border-gray-600 cursor-pointer",
                id: id.clone(),
                r#type: "checkbox",
                checked: *checked.read(),
                value: "",
                onclick: move |_event| {
                    let new_value = !checked.read().clone();
                    *checked.write() = new_value;
                },
            }
            label {
                class: "ms-2 text-sm font-medium text-gray-300",
                r#for: id,
                "{text}"
            }
        }
    }
}
