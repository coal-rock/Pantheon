use yew::prelude::*;

use crate::components::full_page::FullPage;

#[function_component(About)]
pub fn about() -> Html {
    html! {
        <FullPage>
            <h1 style = "font-size:30px; bold;" > { "About" } </h1>
        </FullPage>
    }
}
