use dioxus::prelude::*;

#[component]
pub fn Notepad() -> Element {
    let mut input = use_signal(|| String::new());

    rsx! {
        div {
            class: "bg-zinc-950 w-full h-full rounded-xs flex flex-col p-4 drop-shadow draggable border-2 border-gray-600",
            div {
                class: "text-gray-300 text-xl font-sans pl-1 handle cursor-grab active:cursor-grab",
                "Notepad"
            }
            div {
                class: "flex items-center h-4",
                hr {
                    class: "w-full",
                }
            }
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
