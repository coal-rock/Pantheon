use yew::prelude::*;

use crate::components::full_page::FullPage;

#[function_component(Agent)]
pub fn agent() -> Html {
    html! {
        <FullPage>
            <h1> { "agent" } </h1>
        </FullPage>
    }
}
