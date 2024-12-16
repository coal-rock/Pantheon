mod components;

use patternfly_yew::prelude::*;
use yew::prelude::*;

use components::agent_table::AgentTable;

#[function_component(App)]
fn app() -> Html {
    let brand = html! (
        <MastheadBrand>
            <Title size={Size::XXXXLarge}>
                { "Athena" }
            </Title>
        </MastheadBrand>
    );

    let sidebar = html_nested! {
        <PageSidebar>
            <Nav>
                <NavLink>{"Main Panel"}</NavLink>
                <NavExpandable title="Agents">
                </NavExpandable>
                <NavLink>{"Settings"}</NavLink>
                <NavLink>{"About"}</NavLink>
            </Nav>
        </PageSidebar>
    };

    let tools = html!(
        <Toolbar full_height=true>
            <ToolbarContent>
                <ToolbarGroup
                    modifiers={ToolbarElementModifier::Right.all()}
                >
                    <ToolbarItem>
                        <h5>
                        {"v0.0.1"}
                        </h5>
                    </ToolbarItem>
                </ToolbarGroup>
            </ToolbarContent>
        </Toolbar>
    );

    html! {
    <>

        <Page {brand} {sidebar} {tools}>
            <AgentTable/>
        </Page>
    </>

    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
