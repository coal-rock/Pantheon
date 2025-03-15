use dioxus::prelude::*;

use crate::components::panel_base::PanelBase;

#[component]
pub fn Notepad() -> Element {
    let mut input = use_signal(|| String::new());

    rsx! {
        PanelBase {
            title: "Notepad",
            div {
                class: "flex h-0 grow shrink basis-0 w-full bg-zinc-900 mt-2 rounded p-2",
                div {
                    class: "whitespace-pre text-gray-300 font-mono text-sm overflow-x-scroll no-scrollbar word-break w-full outline-none",
                    div {
                        class: "flex flex-col w-full h-full",
                        textarea {
                            class: "flex w-full h-full resize-none",
                            oninput: move |event| {
                                input.set(event.value());
                            },
                        }
                    }
                }
            }
        }
    }
}
