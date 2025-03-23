use dioxus::prelude::*;
use talaria::console::{Command, CommandContext, NewTarget};

use crate::{components::panel_base::PanelBase, services::api::Api};

#[component]
pub fn Console(id: i32) -> Element {
    // FIXME: sorta cooked flexbox layout
    // FIXME: console doesn't auto-scroll for now - fix?
    // FIXME: conosole implementation is awful, figure out how to use callbacks properly
    let mut console = use_signal(|| talaria::console::Console::new(None));

    // a value of true in this tuple means the command failed
    let mut console_history: Signal<Vec<(bool, String)>> = use_signal(|| vec![]);
    let mut input = use_signal(|| String::new());

    let handle_command = move |event: FormEvent| async move {
        let api = use_context::<Api>();
        let console_history = &mut console_history.write();
        let console = &mut console.write();

        async fn scrollToBottom() {
            let _ = document::eval(
                r#"
            setTimeout(() => {
                const element = document.getElementById("console-line");
                if (element) {
                    element.scrollIntoView({ behavior: "smooth", block: "start" });

                    console.log("no way");
                }
            }, 10);
            "#,
            )
            .await;
        }

        console_history.push((false, console.status_line() + &input.read().clone()));

        let current_target = console.get_target();
        let input = &mut input.write();

        let command = match console.handle_command(input.to_string()) {
            Ok(command) => command,
            Err(err) => {
                console_history.push((true, err.to_string()));
                input.clear();
                scrollToBottom().await;
                return;
            }
        };

        input.clear();

        let response = match api
            .console(CommandContext {
                command: command.clone(),
                current_target: console.get_target(),
            })
            .await
        {
            Ok(response) => response,
            Err(err) => {
                console_history.push((true, err.to_string()));
                scrollToBottom().await;
                return;
            }
        };

        console.set_target(match response.new_target {
            NewTarget::NoTarget => None,
            NewTarget::Target { target } => Some(target),
            NewTarget::NoChange => current_target,
        });

        console_history.push((!response.success, response.output.to_string()));

        if command == Command::Clear {
            console_history.clear();
        }

        scrollToBottom().await;
    };

    rsx! {
        PanelBase {
            title: "Console",
            panel_id: id,
            div {
                class: "flex h-0 grow shrink basis-0 w-full bg-zinc-900 mt-2 rounded p-2",
                div {
                    class: "whitespace-pre text-gray-300 font-mono text-sm overflow-x-scroll no-scrollbar word-break w-full",
                    div {
                        class: "flex flex-col focus-none w-full h-full",
                        div {
                            for (error, entry) in console_history.read().clone() {
                                p {
                                    class: if error {"text-red-500"} else {""},
                                    "{entry} "
                                }
                            }
                        }
                        div {
                            class: "flex flex-row",
                            {console.read().status_line()}
                            form {
                                class: "flex w-full",
                                id: "console-line",
                                onsubmit: handle_command,
                                input {
                                    class: "w-full h-full flex align-start word-break focus:outline-none text-sm font-mono grow",
                                    r#type: "text",
                                    value: "{input}",
                                    oninput: move |event| {
                                        input.set(event.value());
                                    },
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
