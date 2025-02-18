use yew::prelude::*;
use patternfly_yew::prelude::{AlertType, Alert, AlertGroup};

use crate::components::full_page::FullPage;

/// Define properties for DangerAlert
#[derive(Properties, PartialEq)]
pub struct DangerAlertProps {
    #[prop_or_default]
    pub message: String, 
}

/// Define the DangerAlert functional component
#[function_component(DangerAlert)]
pub fn danger_alert(props: &DangerAlertProps) -> Html {
    html! {
        <Alert title="Danger alert" r#type={AlertType::Danger}>
            { &*props.message } 
        </Alert>
    }
}

/// Define properties for InfoAlert
#[derive(Properties, PartialEq)]
pub struct InfoAlertProps {
    #[prop_or_default]
    pub message: String, 
}

/// Define the InfoAlert functional component
#[function_component(InfoAlert)]
pub fn info_alert(props: &InfoAlertProps) -> Html {
    html! {
        <Alert title="Info alert" r#type={AlertType::Info}>
            { &*props.message } 
        </Alert>
    }
}
#[function_component(Alerts)]
pub fn alerts() -> Html {
    html! {
        <FullPage>
            <h1 style="font-size:30px; font-weight:bold;"> { "Alerts" } </h1>

            <p> {"Here are all the Info and Danger alerts generated within the Hermes agents"} </p>

            <AlertGroup>
                <Alert title="Custom alert (Default)">{"Some reason for the alert"}</Alert>
                <Alert title="Success alert" r#type={AlertType::Success}>{"Some reason for the alert"}</Alert>
                <Alert title="Warning alert" r#type={AlertType::Warning}>{"Some reason for the alert"}</Alert>
            </AlertGroup>
                // Using the macros to display Danger and Info alerts
                <DangerAlert message="We have lost access to an Agent!" />
                <InfoAlert message="New Agent has been initialized." />
           
        </FullPage>
    }
}