use dioxus::prelude::*;

use crate::components::agents_overview::AgentsOverview;
use crate::components::agents_table::AgentsTable;
use crate::components::console::Console;
use crate::components::notepad::Notepad;
use crate::services::api::Api;
use crate::views::page::Page;

#[component]
pub fn Home() -> Element {
    let mut text = use_signal(|| String::from("fuck"));

    let fetch_new = move |_| async move {
        let api = use_context::<Api>();
        let mut x = 0;

        loop {
            let res = api.list_agents().await.unwrap();
            text.set(format!("{x}"));
            x += 1;
        }
    };

    rsx! {
        button { onclick: fetch_new, id: "save", "{text}" }
        Page {}
    }
}
