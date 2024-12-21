use yew::prelude::*;

#[derive(Clone, PartialEq)]
struct TerminalState {
    output: Vec<String>,
    current_input: String,
}

#[function_component(Console)]
pub fn console() -> Html {
    let console_history = use_state(|| vec![]);

    let onkeydown = {
        let console_history = console_history.clone();
        Callback::from(move |e: KeyboardEvent| {
            web_sys::console::log_1(&e.key().into());

            if e.key() == "Enter" {
                let input: web_sys::HtmlInputElement = e.target_unchecked_into();
                let value = input.value();

                let mut temp = (*console_history).clone();
                temp.push(value);

                console_history.set(temp.to_vec());
                input.set_value("> "); // Clear the input field if needed
            }

            let input: web_sys::HtmlInputElement = e.target_unchecked_into();
            let value = input.value();

            if value.len() < 2 {
                input.set_value("> ");
            }
        })
    };

    html! {
        <>
            <style>
                {r#"
                .console {
                    height: 100%;
                    width: 100%;
                    margin: 0;
                    background-color: black;
                    color: white;
                    font-family: monospace;
                    border-radius: 8px;
                    padding: 8px;
                }

                .console-history {
                    width: 800px;
                    height: 400px;
                    overflow: scroll;
                    -ms-overflow-style: none;  /* IE and Edge */
                    scrollbar-width: none;  /* Firefox */
                }
                
                /* Chrome */
                .console-history::-webkit-scrollbar {
                    display: none;
                }
                "#}
            </style>

            <div class="console">
                <div class="console-history">
                    { for console_history.iter().map(|i| html!{<p> {i} </p>} )}
                    <input
                        type="text"
                        style="background-color: black; color: white; border: none; width: 100%; font-family: monospace; overflow: scroll; outline: none; border-radius: 8px;"
                        value={"> "}
                        onkeydown={onkeydown}
                    />
                </div>
            </div>
        </>
    }
}
