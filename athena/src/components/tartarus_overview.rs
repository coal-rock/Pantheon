use dioxus::prelude::*;

use crate::components::panel_base::PanelBase;
use crate::services::api::Api;

use bytesize::ByteSize;
use std::time::Duration;

#[component]
pub fn TartarusOverview(id: i32) -> Element {
    let mut cpu_usage = use_signal(|| None);
    let mut core_count = use_signal(|| None);
    let mut memory_used = use_signal(|| None);
    let mut memory_total = use_signal(|| None);
    let mut storage_used = use_signal(|| None);
    let mut storage_total = use_signal(|| None);
    let mut os = use_signal(|| None);
    let mut kernel = use_signal(|| None);
    let mut cpu_name = use_signal(|| None);
    let mut hostname = use_signal(|| None);

    let api = use_context::<Signal<Api>>();

    let fetch_info = move |_| async move {
        loop {
            match api.read().get_tartarus_info().await {
                Ok(info) => {
                    cpu_usage.set(Some(info.cpu_usage));
                    core_count.set(Some(info.core_count));
                    memory_used.set(Some(info.memory_used));
                    memory_total.set(Some(info.memory_total));
                    storage_used.set(Some(info.storage_used));
                    storage_total.set(Some(info.storage_total));
                    os.set(Some(info.os));
                    kernel.set(Some(info.kernel));
                    cpu_name.set(Some(info.cpu_name));
                    hostname.set(Some(info.hostname));
                }
                Err(_) => {}
            }

            async_std::task::sleep(Duration::from_secs(1)).await;
        }
    };

    rsx! {
        // not sure how else to do this, maybe a hack
        div {
            onvisible: fetch_info,
            class: "hidden",
        }
        PanelBase {
            title: "Tartarus Overview",
            panel_id: id,
            div {
                class: "flex flex-col gap-4",
                span {
                    class: "justify-start text-gray-300 text-clamp-base flex w-full h-4 underline underline-offset-2",
                    "Resources:"
                }
                UsageSlider {
                    text: "CPU Usage",
                    center_text: format!("of {} cores", core_count().unwrap_or(0)),
                    value: cpu_usage(),
                }

                UsageSlider {
                    text: "Memory Usage",
                    center_text: format!("{} of {}",
                        ByteSize::b(memory_used().unwrap_or(0)).display().si().to_string(),
                        ByteSize::b(memory_total().unwrap_or(0)).display().si().to_string(),
                        ),
                    value: (memory_used().unwrap_or(0) as f32 / memory_total().unwrap_or(0) as f32) * 100.0,
                }

                UsageSlider {
                    text: "Storage Usage",
                    center_text: format!("{} of {}",
                        ByteSize::b(storage_used().unwrap_or(0)).display().si().to_string(),
                        ByteSize::b(storage_total().unwrap_or(0)).display().si().to_string(),
                        ),
                    value: (storage_used().unwrap_or(0) as f32 / storage_total().unwrap_or(0) as f32) * 100.0,
                }
                hr {
                    class: "p-0 m-0",
                }
                span {
                    class: "justify-start text-gray-300 text-clamp-base flex w-full h-4 underline underline-offset-2 -mt-2",
                    "Host Information:"
                }
                div {
                    Info {
                        name: "CPU(s):",
                        value: cpu_name(),
                    }

                    Info {
                        name: "OS:",
                        value: os(),
                    }
                    Info {
                        name: "Kernel:",
                        value: kernel(),
                    }
                    Info {
                        name: "Hostname:",
                        value: hostname(),
                    }
                }
            }
        }
    }
}

#[component]
fn UsageSlider(text: String, value: Option<f32>, center_text: String) -> Element {
    let (value, value_str) = match value {
        Some(value) => {
            if value.is_nan() {
                (0.0, format!("..."))
            } else {
                (value, format!("{value:.0}%"))
            }
        }
        None => (0.0, format!("...")),
    };

    // flowbite
    rsx! {
        div {
            class: "flex w-full h-full flex-col pt-0 pb-0",
            div {
                class: "flex b-1 contents w-full h-full ",
                span {
                    class: "text-clamp-base font-medium text-gray-300 flex flex-1 justify-start",
                    "{text}"
                }
                span {
                    class: "text-clamp-sm font-medium text-gray-300 flex flex-1 justify-center",
                    "{center_text}"
                }
                span {
                    class: "text-clamp-sm font-medium text-gray-300 flex flex-1 justify-end",
                    "{value_str}"
                }
            }
            div {
                class: "bg-zinc-900 rounded-full",
                div {
                    class: "bg-blue-600 h-2.5 rounded-full transition-all delay-150 duration-300 ease-in-out",
                    width: format!("{value}%"),
                }
            }
        }
    }
}

#[component]
fn Info(name: String, value: Option<String>) -> Element {
    rsx! {
        div {
            class: "text-clamp-sm flex flex-row justify-between",
            span {
                class: "text-gray-300",
                "{name}"
            }
            span {
                class: "text-gray-300",
                { value.unwrap_or(String::from("...")) }
            }
        }
    }
}
