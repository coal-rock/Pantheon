use dioxus::prelude::*;

use crate::components::agent_table::AgentTable;
use crate::views::page::Page;

#[component]
pub fn Home() -> Element {
    rsx! {
        Page {
            div {
                class: "grow bg-stone-600 flex space-between items-center p-2 gap-2",
                AgentTable {},
                AgentTable {},
                AgentTable {},

            }
        }
    }
}
