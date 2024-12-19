use std::rc::Rc;

use gloo_timers::{self, callback::Interval};
use patternfly_yew::prelude::*;
use talaria::api::*;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

use crate::components::sidebar::{brand, sidebar, tools};

#[derive(Properties, PartialEq)]
pub struct FullPageProps {
    pub children: Children,
}

#[function_component(FullPage)]
pub fn full_page(props: &FullPageProps) -> Html {
    let data = use_state(|| vec![]);
    {
        let data = data.clone();

        let fetch_and_update = {
            let data = data.clone();
            move || {
                let data = data.clone();

                spawn_local(async move {
                    let fetched_data: Vec<AgentInfo> =
                        gloo_net::http::Request::get("/admin/api/list_agents")
                            .send()
                            .await
                            .unwrap()
                            .json()
                            .await
                            .unwrap();

                    data.set(fetched_data);
                });
            }
        };

        use_effect_with((), {
            let fetch_and_update = fetch_and_update.clone();

            move |_| {
                fetch_and_update();
                let interval = Interval::new(5000, move || fetch_and_update());
                interval.cancel();
            }
        });
    }

    let brand = brand();
    let sidebar = sidebar(&data);
    let tools = tools();

    html! {
        <Page {brand} {sidebar} {tools}>
            { for props.children.iter() }
        </Page>
    }
}
