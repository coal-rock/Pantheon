use yew::prelude::*;

use crate::components::agent_table::AgentTable;
use crate::components::full_page::FullPage;

#[function_component(Home)]
pub fn home() -> Html {
    html! {
        <FullPage>
            <AgentTable/>
        </FullPage>
    }
}
