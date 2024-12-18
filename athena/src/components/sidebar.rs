use patternfly_yew::prelude::*;
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

pub fn sidebar() -> VChild<PageSidebar> {
    html_nested! {
        <PageSidebar>
            <Nav>
                <Link<Route> to={Route::Home}>
                    <NavLink>{"Main Panel"}</NavLink>
                </Link<Route>>

                <NavExpandable title="Agents"/>

                <Link<Route> to={Route::Settings}>
                    <NavItem>{"Settings"}</NavItem>
                </Link<Route>>

                <Link<Route> to={Route::About}>
                    <NavItem>{"About"}</NavItem>
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
