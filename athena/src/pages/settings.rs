use yew::prelude::*;

use crate::components::full_page::FullPage;

#[function_component(Settings)]
pub fn settings() -> Html {
    html! {
        <FullPage>
            <h1> { "settings" } </h1>
        </FullPage>
    }
}
