use gloo_timers::callback::Interval;
use patternfly_yew::prelude::*;
use talaria::api::*;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

enum Column {}

#[function_component(NetworkLog)]
pub fn network_log() -> Html {
    let data = use_state(|| vec![]);
    {
        let data = data.clone();

        let fetch_and_update = {
            let data = data.clone();
            move || {
                let data = data.clone();

                spawn_local(async move {
                    let fetched_data: Vec<AgentInfo> =
                        gloo_net::http::Request::get("/admin/api/list_agents")
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

        use_effect_with((), {
            let fetch_and_update = fetch_and_update.clone();

            move |_| {
                fetch_and_update();
                let interval = Interval::new(5000, move || fetch_and_update());
                interval.cancel();
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
