use crate::components::console::ConsoleWindow;
use crate::components::network_log::NetworkLog;
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
            <style>
                {r#"
                .container {
                    display: grid;
                    grid-template-columns: repeat(2, 1fr);
                    grid-template-rows: repeat(2, 1fr);
                    grid-column-gap: 12px;
                    grid-row-gap: 12px;
                }
                .box {
                    margin: 8px;
                    height: 500px;
                }
                "#}
            </style>

            <div class="container">
                <Card class="box">
                    <CardTitle> {"Console"} </CardTitle>
                    <CardBody>
                        <ConsoleWindow/>
                    </CardBody>
                </Card>

                <Card class="box"></Card>
                <Card class="box">
                    <CardTitle> {"Network Log"} </CardTitle>
                    <CardBody>
                        <NetworkLog agent_id={agent_id}/>
                    </CardBody>
                </Card>
                <Card class="box"></Card>
            </div>

        </FullPage>
    }
}
