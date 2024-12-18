use yew::prelude::*;

use crate::components::full_page::FullPage;

#[function_component(About)]
pub fn about() -> Html {
    html! {
        <FullPage>
            <h1> { "about" } </h1>
        </FullPage>
    }
}
