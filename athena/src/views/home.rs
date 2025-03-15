use dioxus::prelude::*;

use crate::components::agents_overview::AgentsOverview;
use crate::components::agents_table::AgentsTable;
use crate::components::console::Console;
use crate::components::notepad::Notepad;
use crate::components::tartarus_overview::TartarusOverview;
use crate::views::page::Page;

#[component]
pub fn Home() -> Element {
    rsx! {
        Page {
            div {
                class: "grow bg-zinc-700 flex space-between items-center flex-row gap-2 p-2",
                div {
                    class: "flex flex-col h-full p-0 grow shrink basis-0 w-0 gap-2",
                    AgentsTable {},
                    Console {},
                }
                div {
                    class: "flex flex-col h-full p-0 grow shrink basis-0 w-0 gap-2",
                    AgentsOverview{}
                    TartarusOverview{}
                }
                div {
                    class: "flex flex-col h-full p-0 grow shrink basis-0 w-0 gap-2",
                    Notepad{}
                    TartarusOverview{}
                }
            }
        }
    }
}
