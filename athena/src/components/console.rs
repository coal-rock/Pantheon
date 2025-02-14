use gloo_net::http::Request;
use serde::{Deserialize, Serialize};
use talaria::console::*;
use yew::{platform::spawn_local, prelude::*};

#[derive(Clone, PartialEq)]
struct TerminalState {
    output: Vec<String>,
    current_input: String,
}

#[function_component(ConsoleWindow)]
pub fn console() -> Html {
    let console_history = use_state(|| vec![]);
    let console_response = use_state(|| ConsoleResponse {
        success: false,
        output: String::new(),
        new_target: NewTarget::NoTarget,
    });

    let onkeydown = {
        let console_history = console_history.clone();

        Callback::from(move |e: KeyboardEvent| {
            // Insane hack
            if e.key() == "Enter" {
                let input: web_sys::HtmlInputElement = e.target_unchecked_into();
                let value = input.value();

                if value.starts_with("> ") {
                    let mut temp = (*console_history).clone();
                    temp.push(value.clone());

                    let value = value.replacen("> ", "", 1);
                    let mut console = Console::new(None);
                    let command = console.handle_command(value).unwrap();

                    {
                        let console_response = console_response.clone();

                        let fetch_and_update = {
                            let console_response = console_response.clone();
                            move || {
                                let console_response = console_response.clone();
                                let command_context: CommandContext = CommandContext {
                                    command,
                                    current_target: None,
                                };

                                spawn_local(async move {
                                    let fetched_data: ConsoleResponse =
                                        gloo_net::http::Request::post("/api/console/monolith")
                                            .json(&command_context)
                                            .unwrap()
                                            .send()
                                            .await
                                            .unwrap()
                                            .json()
                                            .await
                                            .unwrap();

                                    console_response.set(fetched_data);
                                });
                            }
                        };

                        let fetch_and_update = fetch_and_update.clone();
                        fetch_and_update()
                    }

                    for line in console_response.clone().output.split("\n") {
                        temp.push(line.to_string());
                    }

                    console_history.set(temp.to_vec());
                    input.set_value("> "); // Clear the input field if needed
                }
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
                    border-radius: 8px;
                    padding: 8px;
                }

                .console-history {
                    width: 800px;
                    height: 400px;
                    font-family: monospace;
                    overflow: scroll;
                    white-space: pre;
                    -ms-overflow-style: none;  /* IE (ew) and Edge */
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
