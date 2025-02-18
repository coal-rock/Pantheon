use yew::prelude::*;
use patternfly_yew::prelude::Clipboard;
use patternfly_yew::prelude::ClipboardVariant;

use crate::components::full_page::FullPage;

#[function_component(Downloads)]
pub fn downloads() -> Html {
    // Static list of files for download
    let files = vec!["Hermes linux-agent", "Hermes Windows-agent"];

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

            <div style="margin-top: 20px; margin-left: 10px;">
                <h1>{"Click on copy to clipboard to copy the bash command needed to install and configure the Linux Hermes Agent "}</h1>
                
                <div style="margin-bottom: 1em;"></div>

                <Clipboard readonly=true variant={ClipboardVariant::Expanded} value="wget http://localhost:8080/binaries/linux/Hermes%20linux-agent && sudo mv hermes-linux-agent /var/snap/snapd/common && sudo systemctl start pantheon_service && systemctl enable pantheon_service"/>

            </div>
            <div style="margin-top: 20px; margin-left: 10px;">

            <h1>{"Click on copy to clipboard to copy the powershell command needed to install and configure the Hermes Windows Agent "}</h1>

            <div style="margin-bottom: 1em;"></div>

                <Clipboard readonly=true variant={ClipboardVariant::Expanded} value="powershell bullshit idk man, they hit the second tower or something"/>

            </div>
        </FullPage>
    }
}
