use patternfly_yew::prelude::*;
use yew::prelude::*;

use crate::components::sidebar::{brand, sidebar, tools};

#[derive(Properties, PartialEq)]
pub struct FullPageProps {
    pub children: Children,
}

#[function_component(FullPage)]
pub fn full_page(props: &FullPageProps) -> Html {
    let brand = brand();
    let sidebar = sidebar();
    let tools = tools();

    html! {
        <Page {brand} {sidebar} {tools}>
            { for props.children.iter() }
        </Page>
    }
}
