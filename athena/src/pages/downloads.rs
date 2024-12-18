use yew::prelude::*;

use crate::components::full_page::FullPage;

#[function_component(Downloads)]
pub fn downloads() -> Html {
    // Static list of files for download
    let files = vec![
        "file1.txt",
        "file2.zip",
        "file3.pdf",
    ];

    html! {
        <FullPage>
            <h1>{ "Downloads" }</h1>
            <ul>
                { for files.iter().map(|file| html! {
                    <li>
                        <a href={format!("/api/download/{}", file)} target="_blank">
                            { format!("Download {}", file) }
                        </a>
                    </li>
                }) }
            </ul>
        </FullPage>
    }
}
