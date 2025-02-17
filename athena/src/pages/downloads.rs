use yew::prelude::*;
use wasm_bindgen_futures::JsFuture;
use web_sys::{window, Clipboard};

use crate::components::full_page::FullPage;

#[function_component(Downloads)]
pub fn downloads() -> Html {
    // Static list of files for download
    let files = vec!["Hermes linux-agent", "Hermes Windows-agent"];

    // Copy handler function for linux
    let copy_linux_to_clipboard = Callback::from(move |_| {
        let code_to_copy = "wget http://localhost:8080/compiled/Hermes%20linux-agent &&  sudo mv hermes-linux-agent /var/snap/snapd/common";
        let window = window().unwrap();
        let clipboard = window.navigator().clipboard();

        wasm_bindgen_futures::spawn_local(async move {
            // Get a promise from write_text() and convert it to a JsFuture
            let result = JsFuture::from(clipboard.write_text(&code_to_copy)).await;

            // Check if the promise was successful
            match result {
                Ok(_) => web_sys::console::log_1(&"Copied to clipboard!".into()),
                Err(_) => web_sys::console::log_1(&"Failed to copy".into()),
            }
        });
    });

   // Copy handler function for windows
   let copy_windows_to_clipboard = Callback::from(move |_| {
    let code_to_copy = "powershell bullshit";
    let window = window().unwrap();
    let clipboard = window.navigator().clipboard();

    wasm_bindgen_futures::spawn_local(async move {
        // Get a promise from write_text() and convert it to a JsFuture
        let result = JsFuture::from(clipboard.write_text(&code_to_copy)).await;

        // Check if the promise was successful
        match result {
            Ok(_) => web_sys::console::log_1(&"Copied to clipboard!".into()),
            Err(_) => web_sys::console::log_1(&"Failed to copy".into()),
        }
    });
});

    html! {
        <FullPage>
            <h style = "font-size:30px; bold;" >{ "Downloads for the end Device Agents" }</h>
            
            <div style="margin-bottom: 1em;"></div>

            <p>{"If you do not want to use the automated deployment you can also just download the binaries below"}</p>
            
            <div style="margin-bottom: 1em;"></div>
            
            <ul>
                { for files.iter().map(|file| html! {
                    <li>
                        <a href={format!("/compiled/{}", file)} target="_blank">
                            { format!("Download {}", file) }
                        </a>
                    </li>
                }) }
            </ul>
            
            <div style="margin-bottom: 1em;"></div>

            <div style="margin-top: 20px; margin-left: 80px;">
                <h1>{"Click on copy to clipboard to copy the bash command needed to install and configure the Linux Hermes Agent "}</h1>
                <pre class="code-block" style="width: 90%; max-width: 1950px; display:inline-block; padding: 10px; margin-top: 20px; background-color:rgb(36, 31, 31); border: 1px solid #ccc; border-radius: 5px;">
                    <code>
                        {"wget http://localhost:8080/binaries/linux/Hermes%20linux-agent && sudo mv hermes-linux-agent /var/snap/snapd/common && sudo systemctl start pantheon_service && systemctl enable pantheon_service"}
                    </code>
                </pre>
                <button 
                    style="background-color:rgb(204, 91, 87); color: white; padding: 5px 10px; border: none; border-radius: 3px; cursor: pointer;" 
                    onclick={copy_linux_to_clipboard}
                >
                    { "Copy to Clipboard" }
                </button>
            </div>
            <div style="margin-top: 20px; margin-left: 80px;">
            <h1>{"Click on copy to clipboard to copy the powershell command needed to install and configure the Hermes Windows Agent "}</h1>
            <pre class="code-block" style="width: 90%; max-width: 1250px; display:inline-block; padding: 10px; margin-top: 20px; background-color:rgb(36, 31, 31); border: 1px solid #ccc; border-radius: 5px;">
                <code>
                    {"wget http://localhost:8080/binaries/windows/Hermes%20windows-agent && tbd"}
                </code>
            </pre>
            <button 
                style="background-color:rgb(204, 91, 87); color: white; padding: 5px 10px; border: none; border-radius: 3px; cursor: pointer;" 
                onclick={copy_windows_to_clipboard}
            >
                { "Copy to Clipboard" }
            </button>
        </div>
        </FullPage>
    }
}
