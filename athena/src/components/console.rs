use dioxus::prelude::*;

use crate::components::panel_base::PanelBase;

#[component]
pub fn Console(id: i32) -> Element {
    // FIXME: sorta cooked flexbox layout
    // FIXME: console doesn't auto-scroll for now - fix?
    let mut command_history: Signal<Vec<String>> = use_signal(|| vec![]);
    let mut input = use_signal(|| String::new());
    let empty_input = use_signal(|| String::new());

    rsx! {
        PanelBase {
            title: "Console",
            panel_id: id,
            div {
                class: "flex h-0 grow shrink basis-0 w-full bg-zinc-900 mt-2 rounded p-2",
                div {
                    class: "whitespace-pre text-gray-300 font-mono text-sm overflow-x-scroll no-scrollbar word-break w-full",
                    div {
                        class: "flex flex-col focus-none w-full h-full",
                        div {
                            for command in command_history.iter() {
                                p { "> {command} "}
                            }
                        }
                        div {
                            class: "flex flex-row",
                            "> ",
                            form {
                                class: "flex w-full",
                                id: "console-line",
                                onsubmit: move |_event| {
                                    command_history.write().push(input.read().clone());
                                    input.set(empty_input());
                                },
                                input {
                                    class: "w-full h-full flex align-start word-break focus:outline-none text-sm font-mono grow",
                                    r#type: "text",
                                    value: "{input}",
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
    }
}
