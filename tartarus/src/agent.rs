use rocket::Route;

#[post("/<agent_id>/register")]
fn register_agent(agent_id: u8) -> String {
    todo!()
}

#[get("/<agent_id>/tasks")]
fn retrieve_tasks(agent_id: u8) -> String {
    todo!()
}

#[post("/<agent_id>/heartbeat")]
fn heartbeat(agent_id: u8) -> String {
    todo!()
}

#[get("/file/<file_id>?<agent_id>")]
fn agent_retrieve_file(agent_id: u8, file_id: u8) -> String {
    todo!()
}

#[post("/file/<file_id>?<agent_id>")]
fn agent_publish_file(agent_id: u8, file_id: u8) -> String {
    todo!()
}

pub fn routes() -> Vec<Route> {
    // routes![]
    todo!()
}
