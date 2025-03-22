use dioxus::prelude::*;

use crate::components::agents_overview::AgentsOverview;
use crate::components::agents_table::AgentsTable;
use crate::components::console::Console;
use crate::components::notepad::Notepad;
use crate::services::api::Api;
use crate::views::page::Page;

#[component]
pub fn Home() -> Element {
    rsx! {
        Page {}
    }
}
