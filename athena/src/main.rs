mod components;
mod pages;

use pages::{about::About, agent::Agent, downloads::Downloads, home::Home, settings::Settings};
use yew::prelude::*;
use yew_router::prelude::*;
use web_sys::{window, UrlSearchParams};

const STATIC_API_TOKEN: &str = "bb123#123"; // The static token

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
    #[at("/denied")]
    AccessDenied,
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
        Route::AccessDenied => html! { <h1>{ "Access Denied - Invalid Token" }</h1> },
        Route::NotFound => html! { <h1>{ "404 - Page not found" }</h1> },
    }
}

#[function_component(App)]
fn app() -> Html {
    let navigator = use_navigator().unwrap();

    use_effect_with((), move |_| {
        let location = window().unwrap().location();
        let search = location.search().unwrap_or_default();
        let params = UrlSearchParams::new_with_str(&search).unwrap();

        // Check if the token is in the URL
        if let Some(token) = params.get("token") {
            if token == STATIC_API_TOKEN {
                // Store the token in localStorage
                window().unwrap().local_storage().unwrap().unwrap().set_item("api_token", &token).unwrap();
            } else {
                // Redirect to Access Denied
                navigator.push(&Route::AccessDenied);
            }
        } else {
            // If no token in URL, check localStorage
            let stored_token = window().unwrap().local_storage().unwrap().unwrap().get_item("api_token").unwrap();
            if stored_token != Some(STATIC_API_TOKEN.to_string()) {
                // No valid token found, redirect
                navigator.push(&Route::AccessDenied);
            }
        }

        || {}
    });

    html! {
        <BrowserRouter>
            <Switch<Route> render={switch} /> // <- must be child of <BrowserRouter>
        </BrowserRouter>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
