mod components;
mod pages;

use pages::{about::About, agent::Agent, downloads::Downloads, home::Home, settings::Settings};

use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Clone, Routable, PartialEq)]
enum Route {
    #[at("/")]
    Home,
    #[at("/agent/:id")]
    Agent { id: u64 },
    #[at("/settings")]
    Settings,
    #[at("/about")]
    About,
    #[at("/downloads")]
    Downloads,
    #[not_found]
    #[at("/404")]
    NotFound,
}

fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! { <Home /> },
        Route::Agent { id } => html! { <Agent agent_id = {id}/> },
        Route::Settings => html! { <Settings /> },
        Route::About => html! { <About /> },
        Route::Downloads => html! { <Downloads /> },
        Route::NotFound => html! { <h1>{ "404 - Page not found" }</h1> },
    }
}

#[function_component(App)]
fn app() -> Html {
    html! {
        <BrowserRouter>
            <Switch<Route> render={switch} /> // <- must be child of <BrowserRouter>
        </BrowserRouter>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
