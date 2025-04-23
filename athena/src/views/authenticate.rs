use std::time::Duration;

use dioxus::prelude::*;
use dioxus_sdk::{
    storage::{use_synced_storage, LocalStorage},
    *,
};

use crate::{components::navbar::Navbar, services::api::Api, Route};

#[component]
pub fn Authenticate() -> Element {
    let mut online = use_signal(|| false);
    let mut authed = use_signal(|| false);

    let host = use_synced_storage::<LocalStorage, String>("host".to_string(), || String::new());
    let token = use_synced_storage::<LocalStorage, String>("token".to_string(), || String::new());

    let mut api = use_context::<Signal<Api>>();

    let fetch_status = move |_| async move {
        loop {
            {
                let mut api = api.write();

                online.set(api.check_host(&host()).await);
                authed.set(api.check_auth(&host(), &token()).await);

                if online() {
                    api.set_api_base(&host());
                    api.set_token(&token());
                }
            }

            async_std::task::sleep(Duration::from_secs(1)).await;
        }
    };

    rsx! {
        div {
            class: "flex flex-col h-screen w-screen",
            onvisible: fetch_status,
            Navbar {
                anemic: true,
            }

            div {
                class: "flex flex-row grow",
                div {
                    class: "w-full h-full flex justify-center items-center bg-zinc-700",
                    div {
                        class: "bg-zinc-950 w-200 h-80 rounded-xs flex flex-col p-4 draggable border-2 border-gray-500",
                        div {
                            class: "text-gray-300 text-2xl font-sans pl-1 flex flex-row justify-between items-center",
                            div {
                                class: "grow",
                                "Authenticate"
                            }
                        }

                        div {
                            class: "flex items-center h-4",
                            hr {
                                class: "w-full",
                            }
                        }
                        div {
                            class: "flex flex-col items-between h-full w-full grow",
                            div {
                                class: "flex flex-col items-center justify-center w-full h-full",
                                TextInput {
                                    label: "Host",
                                    placeholder: "http://localhost:8000",
                                    status: online(),
                                    value: host,
                                }

                                TextInput {
                                    label: "Token",
                                    placeholder: "bb123#123",
                                    status: authed(),
                                    value: token,
                                }
                            }

                            div {
                                class: "flex justify-end items-end h-full",
                                Link {
                                    class: match online() && authed() {
                                        true => "text-white font-medium rounded-lg text-sm px-5 py-2.5 me-2 mb-2 bg-blue-700 hover:bg-blue-800 focus:outline-none dark:focus:ring-blue-800 transition-colors",
                                        false => "text-white font-medium rounded-lg text-sm px-5 py-2.5 me-2 mb-2 bg-gray-700 cursor-not-allowed transition-colors",
                                    },
                                    to: match online() && authed() {
                                        true => {
                                            Route::Home{}
                                        },
                                        false => Route::Authenticate{},
                                    },
                                    "Continue"
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn TextInput(label: String, placeholder: String, status: bool, value: Signal<String>) -> Element {
    rsx! {
        div {
            class: "w-full p-2",
            div {
                class: "text-gray-300 text-lg pb-1",
                {label}
            }
            div {
                class: match status {
                    true => "bg-zinc-900 w-full rounded-xs border-b-1 text-blue-500 h-8 flex items-center pl-2 text-md transition-colors",
                    false => "bg-zinc-900 w-full rounded-xs border-b-1 border-gray-400 h-8 flex items-center pl-2 text-md transition-colors duration-800",
                },
                input {
                    class: "w-full h-full text-gray-300 outline-none",
                    value: value(),
                    oninput: move |event| value.set(event.value()),
                    placeholder,
                }
            }
        }
    }
}
