use gloo_timers::callback::Interval;
use patternfly_yew::prelude::*;
use talaria::api::*;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

use crate::pages::agent::AgentProps;

#[derive(Copy, Clone, Eq, PartialEq)]
enum Column {
    PacketType, // response or instruction
    PacketId,
    PacketName, // command, heartbeat, etc
    PacketData, // "echo"
    Timestamp,
}

impl TableEntryRenderer<Column> for NetworkHistoryEntry {
    fn render_cell(&self, context: CellContext<'_, Column>) -> Cell {
        match context.column {
            Column::PacketType => match self {
                NetworkHistoryEntry::AgentInstruction { instruction: _ } => {
                    html!("Instruction").into()
                }
                NetworkHistoryEntry::AgentResponse { response: _ } => {
                    html!("Response").into() // padding so that rustfmt doesn't kill me
                }
            },
            Column::PacketId => match self {
                NetworkHistoryEntry::AgentInstruction { instruction } => {
                    html!(instruction.packet_header.packet_id.to_string()).into()
                }
                NetworkHistoryEntry::AgentResponse { response } => {
                    html!(response.packet_header.packet_id.to_string()).into()
                }
            },
            Column::PacketName => match self {
                NetworkHistoryEntry::AgentInstruction { instruction } => {
                    html!(instruction.packet_body.variant()).into()
                }
                NetworkHistoryEntry::AgentResponse { response } => {
                    html!(response.packet_body.variant()).into()
                }
            },
            Column::PacketData => match self {
                NetworkHistoryEntry::AgentInstruction { instruction } => {
                    html!(<div style="white-space: pre-wrap;">{instruction.packet_body.inner_value()}</div>).into()
                }
                NetworkHistoryEntry::AgentResponse { response } => {
                    html!(<div style="white-space: pre-wrap;">{response.packet_body.inner_value()}</div>).into()
                },
            },
            Column::Timestamp => match self {
                NetworkHistoryEntry::AgentInstruction { instruction } => html!(instruction.packet_header.timestamp).into(),
                NetworkHistoryEntry::AgentResponse { response } => html!(response.packet_header.timestamp).into(),
            },
        }
    }
}

#[function_component(NetworkLog)]
pub fn network_log(props: &AgentProps) -> Html {
    let agent_id = props.agent_id.clone();

    let data = use_state(|| vec![]);
    {
        let data = data.clone();

        let fetch_and_update = {
            let data = data.clone();
            move || {
                let data = data.clone();

                spawn_local(async move {
                    let fetched_data: Vec<NetworkHistoryEntry> = gloo_net::http::Request::get(
                        &format!("/admin/api/{}/network_history", agent_id).to_string(),
                    )
                    .send()
                    .await
                    .unwrap()
                    .json()
                    .await
                    .unwrap();

                    data.set(fetched_data);
                });
            }
        };

        let fetch_and_update = fetch_and_update.clone();

        use_effect_with((), {
            move |_| {
                fetch_and_update();
                let interval = Interval::new(5000, move || fetch_and_update());

                move || drop(interval)
            }
        });
    }

    let header = html_nested! {
       <TableHeader<Column>>
         <TableColumn<Column> label="Origin" index={Column::PacketType} />
         <TableColumn<Column> label="ID" index={Column::PacketId} />
         <TableColumn<Column> label="Name" index={Column::PacketName} />
         <TableColumn<Column> label="Data" index={Column::PacketData} />
         <TableColumn<Column> label="Timestamp" index={Column::Timestamp} />
       </TableHeader<Column>>
    };

    let (entries, _) = use_table_data(MemoizedTableModel::new((*data).clone().into()));

    html! {
    <div style="width: 800px; height: 400px; overflow: scroll;">
      <Table<Column, UseTableData<Column, MemoizedTableModel<NetworkHistoryEntry>>>
        {header}
        {entries}
      />
    </div>
    }
}
