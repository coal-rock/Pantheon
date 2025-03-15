use dioxus::prelude::*;

#[component]
pub fn PanelBase(children: Element, title: String) -> Element {
    rsx! {
        div {
            class: "bg-zinc-950 w-full h-full rounded-xs flex flex-col p-4 drop-shadow draggable border-2 border-gray-500",
            div {
                class: "text-gray-300 text-xl font-sans pl-1 handle cursor-grab active:cursor-grab",
                "{title}"
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
