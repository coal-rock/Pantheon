use dioxus::prelude::*;

use dioxus_free_icons::icons::fa_brands_icons::{FaLinux, FaWindows};
use dioxus_free_icons::icons::fa_solid_icons::{
    FaArrowLeft, FaArrowRight, FaClock, FaRobot, FaServer,
};
use dioxus_free_icons::Icon;

#[component]
pub fn TartarusOverview() -> Element {
    rsx! {
        div {
            class: "bg-zinc-950 w-full h-full rounded-xs flex flex-col p-4 drop-shadow-xl draggable border-2 border-gray-500",
            div {
                class: "text-gray-300 text-xl font-sans pl-1 handle cursor-grab active:cursor-grab",
                "Tartarus Overview"
            }
            div {
                class: "flex items-center h-4",
                hr {
                    class: "w-full text-color-600",
                }
            }
            div {
                class: "flex flex-col gap-4",
                span {
                    class: "justify-start text-gray-300 text-lg flex w-full h-4 underline underline-offset-2",
                    "Resources:"
                }
                UsageSlider {
                    text: "CPU Usage",
                    center_text: "of 32 cores",
                    value: 34.0,
                }

                UsageSlider {
                    text: "Memory Usage",
                    center_text: "136.14 GiB of 1.69 TiB",
                    value: 8.0,
                }

                UsageSlider {
                    text: "Storage Usage",
                    center_text: "637.86 GiB of 13.79 TiB",
                    value: 5.0,
                }
                hr {
                    class: "p-0 m-0",
                }
                span {
                    class: "justify-start text-gray-300 text-lg flex w-full h-4 underline underline-offset-2 -mt-2",
                    "Host Information:"
                }
                div {
                    Info {
                        name: "CPU(s):",
                        value: "32 x Intel(R) Xeon(R) CPU E5-2690",
                    }
                    Info {
                        name: "Kernel:",
                        value: "Linux 6.8.12-1-pve",
                    }
                    Info {
                        name: "Hostname:",
                        value: "cdo",
                    }
                }
            }
        }
    }
}

#[component]
fn UsageSlider(text: String, value: f64, center_text: String) -> Element {
    let value = format!("{value}%");

    // flowbite
    rsx! {
        div {
            class: "flex w-full h-full flex-col pt-1 pb-1",
            div {
                class: "flex b-1 contents w-full h-full ",
                span {
                    class: "text-base font-medium text-gray-300 flex flex-1 justify-start",
                    "{text}"
                }
                span {
                    class: "text-sm font-medium text-gray-300 flex flex-1 justify-center",
                    "{center_text}"
                }
                span {
                    class: "text-sm font-medium text-gray-300 flex flex-1 justify-end",
                    "{value}"
                }
            }
            div {
                class: "bg-zinc-900 rounded-full",
                div {
                    class: "bg-blue-600 h-2.5 rounded-full",
                    width: value,
                }
            }
        }
    }
}

#[component]
fn Info(name: String, value: String) -> Element {
    rsx! {
        div {
            class: "flex flex-row justify-between",
            span {
                class: "text-gray-300",
                "{name}"
            }
            span {
                class: "text-gray-300",
                "{value}"
            }
        }
    }
}
