use std::time::Duration;

use bytesize::ByteSize;
use dioxus::prelude::*;

use dioxus_free_icons::icons::fa_brands_icons::{FaLinux, FaWindows};
use dioxus_free_icons::icons::fa_solid_icons::{
    FaArrowLeft, FaArrowRight, FaClock, FaRobot, FaServer,
};
use dioxus_free_icons::Icon;

use crate::components::panel_base::PanelBase;
use crate::services::api::Api;

#[component]
pub fn AgentsOverview(id: i32) -> Element {
    let mut registered_agents = use_signal(|| None);
    let mut active_agents = use_signal(|| None);
    let mut packets_sent = use_signal(|| None);
    let mut packets_recv = use_signal(|| None);
    let mut avg_response_latency = use_signal(|| None);
    let mut total_traffic = use_signal(|| None);
    let mut windows_agents = use_signal(|| None);
    let mut linux_agents = use_signal(|| None);

    let api = use_context::<Signal<Api>>();

    let fetch_stats = move |_| async move {
        loop {
            {
                let api = api.read();

                match api.get_tartarus_stats().await {
                    Ok(stats) => {
                        registered_agents.set(Some(stats.registered_agents));
                        active_agents.set(Some(stats.active_agents));
                        packets_sent.set(Some(stats.packets_sent));
                        packets_recv.set(Some(stats.packets_recv));
                        avg_response_latency.set(Some(stats.average_response_latency));
                        total_traffic.set(Some(stats.total_traffic));
                        windows_agents.set(Some(stats.windows_agents));
                        linux_agents.set(Some(stats.linux_agents));
                    }
                    Err(_) => {}
                }
            }

            async_std::task::sleep(Duration::from_secs(1)).await;
        }
    };
    rsx! {
        div {
            onvisible: fetch_stats,
            class: "hidden",
        }
        PanelBase {
            title: "Agent Overview",
            panel_id: id,
            div {
                class: "w-full h-full flex flex-row gap-2 pt-2",
                div {
                    class: "flex flex-col grow shrink basis-0 gap-2",
                    Statistic {
                        text: "Registered Agents:",
                        value: format!("{}", registered_agents().unwrap_or(0)),
                        icon: rsx!{Icon {
                            icon: FaRobot
                        }}
                    }
                    Statistic {
                        text: "Active Agents:",
                        value: format!("{}", active_agents().unwrap_or(0)),
                        icon: rsx!{Icon {
                            icon: FaRobot
                        }}
                    }
                    Statistic {
                        text: "Packets Sent:",
                        value: format!("{}", packets_sent().unwrap_or(0)),
                        icon: rsx!{Icon {
                            icon: FaArrowRight
                        }}
                    }
                    Statistic {
                        text: "Packets Received:",
                        value: format!("{}", packets_recv().unwrap_or(0)),
                        icon: rsx!{Icon {
                            icon: FaArrowLeft,
                        }}
                    }
                }
                div {
                    class: "flex flex-col grow shrink basis-0 gap-2",
                    Statistic {
                        text: "Average Response Latency:",
                        value: avg_response_latency().map(|l| format!("{:.2}ms", l/1000.0)).unwrap_or("?".to_string()),
                        icon: rsx!{Icon {
                            icon: FaClock,
                        }}
                    }
                    Statistic {
                        text: "Total Traffic:",
                        value: ByteSize::b(total_traffic().unwrap_or(0)).display().si().to_string(),
                        icon: rsx!{Icon {
                            icon: FaServer,
                        }}
                    }
                    Statistic {
                        text: "Windows Agents:",
                        value: format!("{}", windows_agents().unwrap_or(0)),
                        icon: rsx!{Icon {
                            icon: FaWindows,
                        }}
                    }
                    Statistic {
                        text: "Linux Agents:",
                        value: format!("{}", linux_agents().unwrap_or(0)),
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
            class: "bg-zinc-900 rounded grow shrink basis-0 text-clamp-lg text-gray-300 flex justify-between items-center",
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
