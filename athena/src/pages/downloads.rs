use yew::prelude::*;

use crate::components::full_page::FullPage;

#[function_component(Downloads)]
pub fn downloads() -> Html {
    // Static list of files for download
    let files = vec![
        "Hermes linux-agent",
        "Hermes Windows-agent",
    ];

    html! {
        <FullPage>
            <h1>{ "Downloads" }</h1>
            <ul>
                { for files.iter().map(|file| html! {
                    <li>
                        <a href={format!("/compiled/{}", file)} target="_blank">
                            { format!("Download {}", file) }
                        </a>
                    </li>
                }) }
            </ul>
        </FullPage>
    }
}
