use patternfly_yew::prelude::*;
use yew::prelude::*;

use crate::components::full_page::FullPage;

#[derive(Properties, PartialEq)]
pub struct AgentProps {
    pub agent_id: u64,
}

#[function_component(Agent)]
pub fn agent(&AgentProps { agent_id }: &AgentProps) -> Html {
    html! {
        <FullPage>
            <div style="display: flex; flex-wrap: wrap; width: 100%; height: 100%; padding: 12px; box-sizing: border-box;">
                <Card style="width: calc(50% - 12px); height: calc(50% - 12px); margin: 6px;">
                    <CardTitle>{"Console"}</CardTitle>

                    <CardBody>
                        <div style="position: relative; height: 90%; width: 100%">
                        <div class="pf-v5-c-code-block" style="position: absolute; top: 0; bottom: 0; left: 0; right: 0;">
                            <div class="pf-v5-c-code-block__content" style="max-height: 100%; overflow-y: auto;">
                                {r#"
                                asldkjasskdlaslkdjaslkdjaklsjdakljsda
                                asldkjasskdlaslkdjaslkdjaklsjdakljsda
                                asldkjasskdlaslkdjaslkdjaklsjdakljsda
                                asldkjasskdlaslkdjaslkdjaklsjdakljsda
                                asldkjasskdlaslkdjaslkdjaklsjdakljsda
                                asldkjasskdlaslkdjaslkdjaklsjdakljsda
                                asldkjasskdlaslkdjaslkdjaklsjdakljsda
                                asldkjasskdlaslkdjaslkdjaklsjdakljsda
                                asldkjasskdlaslkdjaslkdjaklsjdakljsda
                                asldkjasskdlaslkdjaslkdjaklsjdakljsda
                                asldkjasskdlaslkdjaslkdjaklsjdakljsda
                                asldkjasskdlaslkdjaslkdjaklsjdakljsda
                                asldkjasskdlaslkdjaslkdjaklsjdakljsda
                                asldkjasskdlaslkdjaslkdjaklsjdakljsda
                                asldkjasskdlaslkdjaslkdjaklsjdakljsda
                                asldkjasskdlaslkdjaslkdjaklsjdakljsda
                                asldkjasskdlaslkdjaslkdjaklsjdakljsda
                                asldkjasskdlaslkdjaslkdjaklsjdakljsda
                                asldkjasskdlaslkdjaslkdjaklsjdakljsda
                                asldkjasskdlaslkdjaslkdjaklsjdakljsda
                                asldkjasskdlaslkdjaslkdjaklsjdakljsda
                                asldkjasskdlaslkdjaslkdjaklsjdakljsda
                                asldkjasskdlaslkdjaslkdjaklsjdakljsda
                                asldkjasskdlaslkdjaslkdjaklsjdakljsda
                                asldkjasskdlaslkdjaslkdjaklsjdakljsda
                                asldkjasskdlaslkdjaslkdjaklsjdakljsda
                                asldkjasskdlaslkdjaslkdjaklsjdakljsda
                                asldkjasskdlaslkdjaslkdjaklsjdakljsda
                                asldkjasskdlaslkdjaslkdjaklsjdakljsda
                                asldkjasskdlaslkdjaslkdjaklsjdakljsda
                                asldkjasskdlaslkdjaslkdjaklsjdakljsda
                                asldkjasskdlaslkdjaslkdjaklsjdakljsda
                                asldkjasskdlaslkdjaslkdjaklsjdakljsda
                                asldkjasskdlaslkdjaslkdjaklsjdakljsda
                                asldkjasskdlaslkdjaslkdjaklsjdakljsda
                                asldkjasskdlaslkdjaslkdjaklsjdakljsda
                                asldkjasskdlaslkdjaslkdjaklsjdakljsda
                                asldkjasskdlaslkdjaslkdjaklsjdakljsda
                                asldkjasskdlaslkdjaslkdjaklsjdakljsda
                                asldkjasskdlaslkdjaslkdjaklsjdakljsda
                                asldkjasskdlaslkdjaslkdjaklsjdakljsda
                                asldkjasskdlaslkdjaslkdjaklsjdakljsda
                                asldkjasskdlaslkdjaslkdjaklsjdakljsda
                                asldkjasskdlaslkdjaslkdjaklsjdakljsda
                                asldkjasskdlaslkdjaslkdjaklsjdakljsda
                                "#}
                            </div>
                        </div>
                        </div>
                                <TextInputGroup>
                            <TextInputGroupMain
                                placeholder="command"
                                icon={Icon::ArrowRight}
                            />
                        </TextInputGroup>

                    </CardBody>
                </Card>

                <Card style="width: calc(50% - 12px); height: calc(50% - 12px); margin: 6px;">
                    <CardTitle>{"File System"}</CardTitle>
                    <CardBody>{"Content for card 1"}</CardBody>
                </Card>

                <Card style="width: calc(50% - 12px); height: calc(50% - 12px); margin: 6px;">
                    <CardTitle>{"Network Log"}</CardTitle>
                    <CardBody>{"Content for card 1"}</CardBody>
                </Card>

                <Card style="width: calc(50% - 12px); height: calc(50% - 12px); margin: 6px;">
                    <CardTitle>{"Actions"}</CardTitle>
                    <CardBody>{"Content for card 1"}</CardBody>
                </Card>
            </div>
        </FullPage>
    }
}
