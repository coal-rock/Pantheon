use dioxus::prelude::*;

use dioxus_free_icons::icons::fa_brands_icons::{FaLinux, FaWindows};
use dioxus_free_icons::icons::fa_solid_icons::{
    FaArrowLeft, FaArrowRight, FaClock, FaRobot, FaServer,
};
use dioxus_free_icons::Icon;

use crate::components::panel_base::PanelBase;

#[component]
pub fn AgentsOverview(id: i32) -> Element {
    rsx! {
        PanelBase {
            title: "Agent Overview",
            panel_id: id,
            div {
                class: "w-full h-full flex flex-row gap-2 pt-2",
                div {
                    class: "flex flex-col grow shrink basis-0 gap-2",
                    Statistic {
                        text: "Registered Agents:",
                        value: "12",
                        icon: rsx!{Icon {
                            icon: FaRobot
                        }}
                    }
                    Statistic {
                        text: "Active Agents:",
                        value: "10",
                        icon: rsx!{Icon {
                            icon: FaRobot
                        }}
                    }
                    Statistic {
                        text: "Packets Sent:",
                        value: "91822",
                        icon: rsx!{Icon {
                            icon: FaArrowRight
                        }}
                    }
                    Statistic {
                        text: "Packets Received:",
                        value: "91942",
                        icon: rsx!{Icon {
                            icon: FaArrowLeft,
                        }}
                    }
                }
                div {
                    class: "flex flex-col grow shrink basis-0 gap-2",
                    Statistic {
                        text: "Average Response Latency:",
                        value: "25ms",
                        icon: rsx!{Icon {
                            icon: FaClock,
                        }}
                    }
                    Statistic {
                        text: "Total Traffic:",
                        value: "413.1 KB",
                        icon: rsx!{Icon {
                            icon: FaRobot
                        }}
                    }
                    Statistic {
                        text: "Windows Agents:",
                        value: "0",
                        icon: rsx!{Icon {
                            icon: FaWindows,
                        }}
                    }
                    Statistic {
                        text: "Linux Agents:",
                        value: "12",
                        icon: rsx!{Icon {
                            icon: FaLinux,
                        }}
                    }
                }
            }
        }
    }
}

#[component]
fn Statistic(icon: Element, text: String, value: String) -> Element {
    rsx! {
        div {
            class: "bg-zinc-900 rounded grow shrink basis-0 text-lg text-gray-300 flex justify-between items-center",
            div {
                class: "flex flex-row grow shrink basis-0 items-center justify-left",
                div {
                    class: "p-4 pr-2",
                    {icon},
                }
                div {
                    "{text}"
                }
            }
            div {
                class: "p-4",
                "{value}"
            }
        }
    }
}
