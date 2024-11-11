use rocket::Route;

#[post("/<agent_id>/update")]
fn update_agent(agent_id: u8) -> String {
    todo!()
}

#[post("/<agent_id>/deactivate")]
fn deactivate_agent(agent_id: u8) -> String {
    todo!()
}

#[post("/<agent_id>/activate")]
fn activate_agent(agent_id: u8) -> String {
    todo!()
}

#[get("/<agent_id>/get_file?<file_id>")]
fn retrieve_file(agent_id: u8, file_id: u8) -> String {
    todo!()
}

#[get("/<agent_id>/list_files")]
fn list_files(agent_id: u8) -> String {
    todo!()
}

#[post("/<agent_id>/uninstall")]
fn uninstall_agent(agent_id: u8) -> String {
    todo!()
}

#[get("/agents")]
fn list_agents() -> String {
    todo!()
}

#[get("/log")]
fn get_log() -> String {
    todo!()
}

#[get("/<agent_id>/log")]
fn get_log_agent(agent_id: u8) -> String {
    todo!()
}

#[post("/escape_hatch")]
fn escape_hatch() -> String {
    todo!()
}

pub fn routes() -> Vec<Route> {
    // routes![]
    todo!()
}
