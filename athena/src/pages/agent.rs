use yew::prelude::*;

use crate::components::full_page::FullPage;

#[derive(Properties, PartialEq)]
pub struct AgentProps {
    pub agent_id: u64,
}

#[function_component(Agent)]
pub fn agent(&AgentProps { agent_id }: &AgentProps) -> Html {
    html! {
        <FullPage>
            <h1> { agent_id } </h1>
        </FullPage>
    }
}
