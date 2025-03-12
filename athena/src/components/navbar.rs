use dioxus::prelude::*;

use dioxus_free_icons::icons::fa_brands_icons::FaGithub;
use dioxus_free_icons::icons::fa_solid_icons::FaBars;
use dioxus_free_icons::Icon;

use crate::Route;

#[component]
pub fn Navbar(show_sidebar: Signal<bool>) -> Element {
    // we have to use use_signal for this as i'm not sure to apply css
    // to Icons direct
    let mut sidebar_toggle_hover = use_signal(|| false);
    let mut github_hover = use_signal(|| false);

    rsx! {
        div {
            class: "bg-zinc-950 h-16 flex items-center justify-between border-b-2",
            div {
                class: "flex flex-row p-3 items-center",
                button {
                    onmouseenter: move |_event| {
                        sidebar_toggle_hover.set(true);
                    },
                    onmouseleave: move |_event| {
                        sidebar_toggle_hover.set(false);
                    },
                    onclick: move |_event| {
                        let new_value = !show_sidebar.read().clone();
                        *show_sidebar.write() = new_value;
                    },
                    Icon {
                        width: 28,
                        height: 28,
                        fill: if *sidebar_toggle_hover.read() {"white"} else {"lightgray"},
                        icon: FaBars,
                    }
                }
                div {
                    class: "flex flex-col p-4",
                    Link {
                        class: "text-white font-sans text-4xl",
                        to: Route::Home {},
                        "Athena"
                    }
                    h1 {
                        class: "text-gray-400 font-sans text-sm",
                        "v0.0.1"
                    }
                }
            }
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
                    class: "pr-2",
                    width: 52,
                    height: 52,
                    fill: if *github_hover.read() {"white"} else {"lightgray"},
                    icon: FaGithub,
                }
            }
        }
    }
}
