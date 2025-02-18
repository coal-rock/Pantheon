use gloo_timers;
use patternfly_yew::prelude::*;
use talaria::api::*;
use wasm_bindgen_futures::spawn_local;
use yew::{prelude::*, virtual_dom::VChild};
use yew_router::prelude::*;

use crate::Route;

pub fn brand() -> Html {
    html! {
        <MastheadBrand>
            <Title size={Size::XXXXLarge}>
                { "Athena" }
            </Title>
        </MastheadBrand>
    }
}

pub fn sidebar(agents: &Vec<AgentInfo>) -> VChild<PageSidebar> {
    html_nested! {
        <PageSidebar>
            <Nav>
                <Link<Route> to={Route::Home}>
                    <NavLink>{"Main Panel"}</NavLink>
                </Link<Route>>

                <NavExpandable title="Agents">
                    {
                        agents.into_iter().map(|agent| {
                            html! {
                                <Link<Route> to={Route::Agent { id: agent.id }}>
                                    <NavItem>
                                        <div style="display: block">
                                            <div style="font-size: 14px !important;">
                                                {agent.name.clone()}
                                            </div>

                                            <div style="color: grey !important;">
                                                {agent.id}
                                            </div>
                                        </div>
                                    </NavItem>
                                </Link<Route>>
                            }
                        }).collect::<Html>()
                    }
                </NavExpandable>

                <Link<Route> to={Route::Settings}>
                    <NavItem>{"Settings"}</NavItem>
                </Link<Route>>

                <Link<Route> to={Route::About}>
                    <NavItem>{"About"}</NavItem>
                </Link<Route>>

                <Link<Route> to={Route::Downloads}>
                    <NavItem>{"Downloads"}</NavItem>
                </Link<Route>>

                <Link<Route> to={Route::Alerts}>
                    <NavItem>{"Alerts"}</NavItem>
                </Link<Route>>

            </Nav>
        </PageSidebar>
    }
}

pub fn tools() -> Html {
    html! {
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
    }
}
