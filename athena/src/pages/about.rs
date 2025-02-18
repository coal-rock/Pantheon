use yew::prelude::*;
use patternfly_yew::prelude::CodeBlockCode;
use patternfly_yew::prelude::CodeBlock;

use crate::components::full_page::FullPage;

#[function_component(About)]
pub fn about() -> Html {
    html! {
        <FullPage>
            <h1 style = "font-size:30px; bold;" > { "About" } </h1>

            <div style="margin-bottom: 1em;"></div>
            <div style="margin-bottom: 1em;"></div>

                <CodeBlock>
                    <CodeBlockCode>
                    {r#" fortnite poop balls... aka fix this in post"#}
                    </CodeBlockCode>
                </CodeBlock>
        </FullPage>
    }
}
