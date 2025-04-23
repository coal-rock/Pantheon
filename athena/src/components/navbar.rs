use dioxus::prelude::*;

use dioxus_free_icons::icons::fa_brands_icons::FaGithub;
use dioxus_free_icons::Icon;

use crate::views::page::PanelManager;
use crate::Route;

#[component]
pub fn Navbar(anemic: bool) -> Element {
    // we have to use use_signal for this as i'm not sure to apply css
    // to Icons direct
    let mut github_hover = use_signal(|| false);

    rsx! {
        div {
            class: "bg-zinc-950 h-16 flex items-center justify-between border-0 border-b-2 border-gray-600",
            div {
                class: "flex flex-row items-center w-64",
                div {
                    class: "flex flex-row border-l-0 border-2 border-gray-600 items-center justify-center",
                    div {
                        class: "w-12 h-full",
                        img {
                            src: asset!("assets/cdo-logo.png"),
                            width: 50,
                            height: 50,
                        }
                    }
                    div {
                        class: "flex flex-col p-2",
                        Link {
                            class: "text-gray-300 hover:text-white font-sans text-4xl",
                            to: Route::Home{},
                            "Athena"
                        }
                        h1 {
                            class: "text-gray-400 font-sans text-sm",
                            "v0.0.1"
                        }
                    }
                }

                div {
                    class: "grow h-full flex flex-row gap-0 justify-left items-left",
                    div {
                        class: "p-2 border-r-2 border-gray-600 flex flex-col w-24 h-full items-center",
                        div {
                            class: "text-gray-300 text-md",
                            "Tartarus"
                        }
                        div {
                            class: "text-gray-400 text-sm",
                            "192.168.1.2"
                        }
                    }
                    div {
                        class: "p-2 border-r-2 border-gray-600 flex flex-col w-20 h-full items-center",
                        div {
                            class: "text-gray-300 text-md",
                            "Uptime"
                        }
                        div {
                            class: "text-gray-400 text-sm",
                            "100 hours"
                        }
                    }

                    div {
                        class: "p-2 border-r-2 border-gray-600 flex flex-col w-16 h-full items-center hover:bg-zinc-900 cursor-pointer relative inline-block dropdown",
                        LayoutMenu {
                            LayoutElement {
                                layout: vec![1],
                            }

                            LayoutElement {
                                layout: vec![2]
                            }

                            hr{}

                            LayoutElement {
                                layout: vec![1, 1]
                            }

                            LayoutElement {
                                layout: vec![2, 2]
                            }

                            hr{}

                            LayoutElement {
                                layout: vec![1, 1, 1]
                            }

                            LayoutElement {
                                layout: vec![2, 2, 2]
                            }
                        }
                    }

                    div {
                        class: "p-2 border-r-2 border-gray-600 flex flex-col w-16 h-full items-center hover:bg-zinc-900 cursor-pointer relative inline-block dropdown",
                        LayoutMenu {
                            LayoutElement {
                                layout: vec![1],
                            }

                            LayoutElement {
                                layout: vec![2]
                            }

                            hr{}

                            LayoutElement {
                                layout: vec![1, 1]
                            }

                            LayoutElement {
                                layout: vec![2, 2]
                            }

                            hr{}

                            LayoutElement {
                                layout: vec![1, 1, 1]
                            }

                            LayoutElement {
                                layout: vec![2, 2, 2]
                            }
                        }
                    }
                }
            }

            div {
                class: "h-full p-2 border-l-2 border-gray-600",
                a {
                    href: "https://github.com/Dack985/Pantheon",
                    target: "_blank",
                    onmouseenter: move |_event| {
                        github_hover.set(true);
                    },
                    onmouseleave: move |_event| {
                        github_hover.set(false);
                    },
                    Icon {
                        class: "w-full h-full",
                        fill: if *github_hover.read() {"white"} else {"lightgray"},
                        icon: FaGithub,
                    }
                }
            }
        }
    }
}

#[derive(Clone)]
struct Hidden(Signal<bool>);

#[component]
fn LayoutMenu(children: Element) -> Element {
    let mut hidden = use_context_provider(|| Hidden(Signal::new(true)));
    let hidden_class = if hidden.0() { "hidden" } else { "block" };

    let panel_manager = use_context::<PanelManager>();

    rsx! {
        div {
            onmouseenter: move |_| {
                hidden.0.set(false);
            },
            onmouseleave: move |_| {
                hidden.0.set(true);
            },
            button {
                id: "dropdown-button",
                div {
                    class: "text-gray-300 text-md",
                    "Layout"
                }
                div {
                    class: "text-gray-400 text-sm",
                    {panel_manager.stringify_layout()}
                }
            }

            ul {
                class: "dropdown-menu absolute pt-4 left-1/2 transform -translate-x-1/2 w-20 z-10 {hidden_class}",
                div {
                    class: "border-gray-600 border-2",
                    {children}
                }
            }
        }
    }
}

#[component]
fn LayoutElement(layout: Vec<i32>) -> Element {
    let mut hidden = use_context::<Hidden>().0;
    let mut panel_manager = use_context::<PanelManager>();

    rsx! {
        li {
            a {
                class: "py-1 px-1 block whitespace-no-wrap bg-zinc-950 text-center hover:bg-zinc-900 text-gray-300",
                onclick: move |_| {
                    hidden.set(true);
                    panel_manager.set_layout(layout.clone());
                },
                {PanelManager::stringify_external_layout(layout.clone())}
            }
        }
    }
}
