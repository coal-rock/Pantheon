use patternfly_yew::prelude::*;
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
                <Example/>

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

#[derive(Clone)]
struct Agent {
    name: String,
    id: String,
    ip: String,
    status: Status,
    ping: i32,
}

#[derive(Clone)]
enum Status {
    Online,
    Offline,
}

impl TableEntryRenderer<Column> for Agent {
    fn render_cell(&self, context: CellContext<'_, Column>) -> Cell {
        match context.column {
            Column::Name => html!(&self.name).into(),
            Column::ID => html!(&self.id).into(),
            Column::IP => html!(&self.ip).into(),
            Column::Status => match self.status {
                Status::Online => html!(<Label label="Online" color={Color::Green}/>).into(),
                Status::Offline => html!(<Label label="Offline" color={Color::Red}/>).into(),
            },
            Column::Ping => html!({ self.ping.to_string() + "ms" }).into(),
            Column::Action => html!(<Button> {"Interact"} </Button>).into(),
        }
    }
}

#[function_component(Example)]
pub fn example() -> Html {
    let entries = use_memo((), |()| {
        vec![
            Agent {
                name: "coal".into(),
                id: "alksdjaslkdj".into(),
                ip: "192.168.0.1".into(),
                status: Status::Online,
                ping: 64,
            },
            Agent {
                name: "fortnite".into(),
                id: "aslkdajsdlkj".into(),
                ip: "127.0.0.1".into(),
                status: Status::Offline,
                ping: 69,
            },
        ]
    });

    let (entries, _) = use_table_data(MemoizedTableModel::new(entries));

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

    html! (
      <Table<Column, UseTableData<Column, MemoizedTableModel<Agent>>>
        {header}
        {entries}
      />
    )
}
