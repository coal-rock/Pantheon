use crate::console_lib;
use crate::SharedState;
use rocket::{serde::json::Json, Route};
use talaria::console::*;

#[post("/monolith", data = "<command_context>")]
pub async fn monolith(
    state: &rocket::State<SharedState>,
    command_context: Json<CommandContext>,
) -> Json<ConsoleResponse> {
    println!("the south monolith has been hit");
    Json(console_lib::evaluate_command(state, command_context.0).await)
}

pub fn routes() -> Vec<Route> {
    rocket::routes![monolith]
}
