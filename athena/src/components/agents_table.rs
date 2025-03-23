use std::time::Duration;

use dioxus::prelude::*;

use dioxus_free_icons::icons::fa_brands_icons::{FaLinux, FaWindows};
use dioxus_free_icons::icons::fa_solid_icons::{FaMagnifyingGlass, FaQuestion};
use dioxus_free_icons::Icon;
use talaria::api::*;
use talaria::protocol::OSType;

use crate::components::panel_base::PanelBase;
use crate::services::api::Api;

#[component]
pub fn AgentsTable(id: i32) -> Element {
    let show_windows = use_signal(|| true);
    let show_linux = use_signal(|| true);
    let show_other = use_signal(|| true);
    let show_inactive = use_signal(|| true);
    let mut query = use_signal(|| String::new());

    let mut agents: Signal<Vec<AgentInfo>> = use_signal(|| vec![]);

    let fetch_agents = move |_| async move {
        let api = use_context::<Api>();

        loop {
            match api.list_agents().await {
                Ok(response) => agents.set(response),
                Err(_) => {}
            }

            async_std::task::sleep(Duration::from_secs(1)).await;
        }
    };
    rsx! {
        div {
            onvisible: fetch_agents,
            class: "hidden",
        }
        PanelBase {
            title: "Agents Table",
            panel_id: id,
            div {
                class: "bg-zinc-900 w-full rounded-xs border-b-1 border-gray-400 h-8 flex items-center pl-2",
                Icon {
                    width: 16,
                    icon: FaMagnifyingGlass,
                    fill: "lightgray"
                }
                input {
                    class: "w-full h-full text-gray-300 outline-none pl-2",
                    value: query(),
                    oninput: move |event| query.set(event.value()),
                    placeholder: "agent name",
                }
            }
            div {
                class: "flex flex-row gap-2 pt-2",
                Checkbox{
                    id: "show-windows",
                    text: "Show Windows",
                    checked: show_windows,
                }
                "|"
                Checkbox{
                    id: "show-linux",
                    text: "Show Linux",
                    checked: show_linux,
                }
                "|"
                Checkbox{
                    id: "show-other",
                    text: "Show Other",
                    checked: show_other,
                }
                "|"
                Checkbox{
                    id: "show-inactive",
                    text: "Show Inactive",
                    checked: show_inactive,
                }
            }
            AgentList{
                agents: agents(),
                show_windows: show_windows(),
                show_linux: show_linux(),
                show_inactive: show_inactive(),
                show_other: show_other(),
                query: query(),
            }
        }
    }
}

#[component]
fn Checkbox(text: String, id: String, checked: Signal<bool>) -> Element {
    // Flowbite
    rsx! {
        div {
            class: "flex items-center",
            input {
                class: "w-4 h-4 text-blue-600 focus:ring-blue-600 ring-offset-gray-800 focus:none bg-gray-700 border-gray-600 cursor-pointer",
                id: id.clone(),
                r#type: "checkbox",
                checked: *checked.read(),
                value: "",
                onclick: move |_event| {
                    let new_value = !checked.read().clone();
                    *checked.write() = new_value;
                },
            }
            span {
                class: "ms-2 text-sm font-medium text-gray-300 select-none",
                "{text}"
            }
        }
    }
}

#[component]
fn AgentList(
    agents: Vec<AgentInfo>,
    show_windows: bool,
    show_linux: bool,
    show_other: bool,
    show_inactive: bool,
    query: String,
) -> Element {
    let agents: Vec<AgentInfo> = agents
        .into_iter()
        .filter(|agent| match agent.os.os_type {
            OSType::Windows => show_windows,
            OSType::Linux => show_linux,
            OSType::Other => show_other,
        })
        .filter(|agent| agent.status || show_inactive)
        .filter(|agent| {
            if query == "" {
                return true;
            }

            match &agent.name {
                Some(name) => name.to_lowercase().contains(&query.to_lowercase()),
                None => false,
            }
        })
        .collect();

    rsx! {
        div {
            class: "flex flex-row h-12 bg-zinc-900 w-full shrink-0 m-0 mt-2 items-center p-2 text-gray-300 text-md justify-between text-center",
            h1 {
                class: "grow-1 shrink basis-0",
                "OS"
            }
            h1 {
                class: "grow-4 shrink basis-0",
                "Name"
            }
            h1 {
                class: "grow-6 shrink basis-0",
                "ID"
            }
            h1 {
                class: "grow-5 shrink basis-0",
                "IP"
            }
            h1 {
                class: "grow-4 shrink basis-0",
                "Status"
            }
            h1 {
                class: "grow-2 shrink basis-0",
                "Ping"
            }
        }
        hr{}
        div {
            class: "flex flex-col w-full h-full mt-0 bg-zinc-950 p-0 gap-0 overflow-x-scroll grow shrink basis-0 no-scrollbar",
            for agent in agents {
                div {
                    class: "flex flex-row h-12 bg-zinc-900 w-full shrink-0 m-0 items-center p-2 text-gray-300 text-md justify-between text-center",
                    div {
                        class: "grow-1 shrink basis-0 flex justify-center",
                        match agent.os.os_type {
                            OSType::Windows =>
                                rsx!{Icon {
                                    icon: FaWindows,
                                }},
                            OSType::Linux =>
                                rsx!{Icon {
                                    icon: FaLinux,
                                }},
                            OSType::Other =>
                                rsx!{Icon {
                                    icon: FaQuestion,
                                }},
                        }
                    }
                    h1 {
                        class: "grow-4 shrink basis-0",
                        {agent.name.unwrap_or("...".to_string())}
                    }
                    h1 {
                        class: "grow-6 shrink basis-0",
                        {agent.id.to_string()}
                    }
                    h1 {
                        class: "grow-5 shrink basis-0",
                        {agent.ip}
                    }
                    div {
                        class: "grow-4 shrink basis-0 flex flex-row items-center justify-center",
                        match agent.status {
                            true => rsx! {
                                div {
                                    class: "h-2.5 w-2.5 rounded-full bg-green-500 me-2"
                                }
                                "Online"
                            },
                            false=> rsx !{
                                div {
                                    class: "h-2.5 w-2.5 rounded-full bg-red-500 me-2"
                                }
                                "Offline"
                            }
                        }
                    }
                    h1 {
                        class: "grow-2 shrink basis-0",
                        {agent.ping.to_string() + "ms"}
                    }
                }
                hr{}
            }
        }
    }
}
