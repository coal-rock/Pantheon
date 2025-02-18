use yew::prelude::*;
use web_sys::HtmlInputElement;
use patternfly_yew::prelude::Button;
use patternfly_yew::prelude::ButtonVariant;

use crate::components::full_page::FullPage;

#[function_component(Settings)]
pub fn settings() -> Html {
    // State to track selected mode (either IP mode or DNS mode)
    let mode = use_state(|| "ip-mode".to_string());

    // Callback to update the mode when the user selects a new option
    let on_mode_change = {
        let mode = mode.clone();
        Callback::from(move |event: Event| {
            let input: HtmlInputElement = event.target_unchecked_into();
            mode.set(input.value());
        })
    };

    let input_value = use_state(|| String::new());

    // Callback to update the input value
    let on_input_change = {
        let input_value = input_value.clone();
        Callback::from(move |event: InputEvent| {
            let input: HtmlInputElement = event.target_unchecked_into();
            input_value.set(input.value());
        })
    };


    html! {
        <FullPage>
            <h1 style="font-size:30px; font-weight: bold;"> { "Settings" } </h1>
        
            <div style="margin-bottom: 1em;"></div>

            <p>{"Select which mode you want to configure. DNS mode configures tartarus to have agents connect via a DNS hostname, and IP mode configures it to use a public or private IP address"}</p>
            
            <div style="margin-bottom: 1em;"></div>

            <div>
                <div class="parent">
                    <select name="status" onchange={on_mode_change}>
                        <option value="ip-mode" selected={*mode == "ip-mode"}>{ "IP mode" }</option>
                        <option value="domain-name-mode" selected={*mode == "domain-name-mode"}>{ "Domain-name mode" } </option> 
                    </select>
                </div>
            </div>

            {
                if *mode == "ip-mode" {
                    html! {
                        <div>
                            <label for="ip-address">{"IP Address:"}</label>
                            <div class="input-container">
                                <input
                                    type="text"
                                    id="ip-address"
                                    value={(*input_value).clone()}
                                    oninput={on_input_change}
                                    placeholder="Enter IP Address"
                                />
                                <Button variant={ButtonVariant::Control}>{ "Enter" }</Button>{" "}
                            </div>
                        </div>
                    }
                } else {
                    html! {
                        <div>
                            <label for="dns-hostname">{"DNS Hostname:"}</label>
                            <div class="input-container">
                                <input
                                    type="text"
                                    id="dns-hostname"
                                    value={(*input_value).clone()}
                                    oninput={on_input_change}
                                    placeholder="Enter DNS Hostname"
                                />
                                <Button variant={ButtonVariant::Control}>{ "Enter" }</Button>{" "}
                            </div>
                        </div>
                    }
                }
            }
        </FullPage>
    }
}