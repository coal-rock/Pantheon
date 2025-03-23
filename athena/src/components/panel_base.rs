use dioxus::prelude::*;

use dioxus_free_icons::icons::fa_solid_icons::FaX;
use dioxus_free_icons::Icon;

use crate::views::page::PanelManager;

#[component]
pub fn PanelBase(children: Element, title: String, panel_id: i32) -> Element {
    let mut panel_manager = use_context::<PanelManager>();
    let mut close_hovered = use_signal(|| false);

    rsx! {
        div {
            id: panel_id.to_string(),
            class: "bg-zinc-950 w-full h-full rounded-xs flex flex-col p-4 drop-shadow draggable border-2 border-gray-500",
            div {
                class: "text-gray-300 text-xl font-sans pl-1 flex flex-row justify-between items-center",
                div {
                    class: "handle grow active:cursor-grab cursor-grab",
                    "{title}"
                }
                div {
                    button {
                        onclick: move |_event| {
                            panel_manager.remove_panel(panel_id);

                            panel_manager.layout.write();
                            panel_manager.open_panels.write();


                            let _ = use_resource(move || async move { document::eval(&format!("document.getElementById(\"{}\").outerHTML = \"\"", panel_id)).await });
                        },
                        onmouseenter: move |_event| {
                            close_hovered.set(true);
                        },
                        onmouseleave: move |_event| {
                            close_hovered.set(false);
                        },
                        Icon {
                            icon: FaX,
                            width: 16,
                            height: 14,
                            fill: if close_hovered() {"#3584e4"} else {"white"}
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
