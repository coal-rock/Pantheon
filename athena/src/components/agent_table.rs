use gloo_timers::{self, callback::Interval};
use patternfly_yew::prelude::*;
use talaria::api::*;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

#[function_component(AgentTable)]
pub fn agent_table() -> Html {
    html! {
        <div style="padding: 32px;">
        <Flex>
        <FlexItem>
        <Card>
            <CardTitle>{"Agents"}</CardTitle>

            <CardBody>
                <div style="display: flex; flex-direction: column; gap: 12px; padding: 12px; padding-top: 0px">
                <TextInputGroup>
                    <TextInputGroupMain
                        placeholder="agent name"
                        icon={Icon::Search}
                    />
                    <TextInputGroupUtilities>
                        <Button icon={Icon::Times} variant={ButtonVariant::Plain} />
                    </TextInputGroupUtilities>
                </TextInputGroup>

                <Switch label="only show active"/>
                </div>


                <Divider r#type={DividerType::Hr} />
                <AgentTableInner/>

            </CardBody>
        </Card>
        </FlexItem>
        </Flex>
        </div>
    }
}

#[derive(Copy, Clone, Eq, PartialEq)]
enum Column {
    Name,
    ID,
    IP,
    Status,
    Ping,
    Action,
}

impl TableEntryRenderer<Column> for AgentInfo {
    fn render_cell(&self, context: CellContext<'_, Column>) -> Cell {
        match context.column {
            Column::Name => html!(&self.name).into(),
            Column::ID => html!(&self.id).into(),
            Column::IP => html!(&self.ip).into(),
            Column::Status => match self.status {
                true => html!(<Label label="Online" color={Color::Green}/>).into(),
                false => html!(<Label label="Offline" color={Color::Red}/>).into(),
            },
            Column::Ping => html!({ self.ping.to_string() + "ms" }).into(),
            Column::Action => html!(<Button> {"Interact"} </Button>).into(),
        }
    }
}

#[function_component(AgentTableInner)]
pub fn agent_table_inner() -> Html {
    let data = use_state(|| vec![]);
    {
        let data = data.clone();

        let fetch_and_update = {
            let data = data.clone();
            move || {
                let data = data.clone();

                spawn_local(async move {
                    let fetched_data: Vec<AgentInfo> =
                        gloo_net::http::Request::get("/api/admin/list_agents")
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
         <TableColumn<Column> label="Name" index={Column::Name} />
         <TableColumn<Column> label="ID" index={Column::ID} />
         <TableColumn<Column> label="IP" index={Column::IP} />
         <TableColumn<Column> label="Status" index={Column::Status} />
         <TableColumn<Column> label="Ping" index={Column::Ping} />
         <TableColumn<Column> label="Action" index={Column::Action} />
       </TableHeader<Column>>
    };

    let (entries, _) = use_table_data(MemoizedTableModel::new((*data).clone().into()));

    html! {
      <Table<Column, UseTableData<Column, MemoizedTableModel<AgentInfo>>>
        {header}
        {entries}
      />
    }
}
