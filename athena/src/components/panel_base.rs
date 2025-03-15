use dioxus::prelude::*;

use dioxus_free_icons::icons::fa_solid_icons::FaX;
use dioxus_free_icons::Icon;

#[component]
pub fn PanelBase(children: Element, title: String) -> Element {
    let mut display_class = use_signal(|| String::new());

    rsx! {
        div {
            class: "bg-zinc-950 w-full h-full rounded-xs flex flex-col p-4 drop-shadow draggable border-2 border-gray-500 {display_class}",
            div {
                class: "text-gray-300 text-xl font-sans pl-1 flex flex-row justify-between items-center",
                div {
                    class: "handle grow active:cursor-grab cursor-grab",
                    "{title}"
                }
                div {
                    button {
                        onclick: move |_event| {
                            *display_class.write() = String::from("hidden");
                        },
                        Icon {
                            icon: FaX,
                            width: 16,
                            height: 14,
                            fill: "red",
                        }
                    }
                }
            }
            div {
                class: "flex items-center h-4",
                hr {
                    class: "w-full",
                }
            }
            {children}
        }
    }
}
