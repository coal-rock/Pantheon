use crate::SharedState;
use rocket::serde::json::Json;
use std::{collections::HashMap, path::Path};
use sysinfo::{Disks, System};
use talaria::api::*;

/// Retrieves all agents
#[get("/agents")]
pub async fn get_agents(state: &rocket::State<SharedState>) -> Json<HashMap<u64, Agent>> {
    Json(state.read().await.agents.clone())
}

/// Retrieves arbitrary amount of network history
/// for specified agent
#[get("/<agent_id>/network_history?<count>")]
pub async fn get_agent_history(
    state: &rocket::State<SharedState>,
    agent_id: u64,
    count: usize,
) -> Option<Json<Vec<NetworkHistoryEntry>>> {
    let agents = state.read().await.agents.clone();

    // FIXME: slow and evil
    match agents.get(&agent_id) {
        Some(agent) => Some(Json(
            agent
                .network_history
                .clone()
                .iter()
                .rev()
                .take(count)
                .map(|x| x.clone())
                .collect::<Vec<NetworkHistoryEntry>>(),
        )),
        None => None,
    }
}

// Retrieves basic info about agent
#[get("/list_agents")]
pub async fn list_agents(state: &rocket::State<SharedState>) -> Json<Vec<AgentInfo>> {
    let agents: HashMap<u64, Agent> = state.read().await.agents.clone();
    let mut agent_info: Vec<AgentInfo> = vec![];

    for (_, agent) in agents {
        agent_info.push(AgentInfo {
            name: agent.nickname.unwrap_or("No Name".to_string()),
            id: agent.id,
            ip: agent.ip.to_string(),
            status: true,
            ping: agent.last_packet_send - agent.last_packet_recv, // FIXME: unsafe,
        });
    }

    Json(agent_info)
}

#[get("/tartarus_info")]
pub async fn tartarus_info(state: &rocket::State<SharedState>) -> Json<TartarusInfo> {
    let mut sys = System::new_all();
    sys.refresh_all();

    let cpu_name = if let Some(cpu) = sys.cpus().first() {
        Some(cpu.brand())
    } else {
        None
    };

    let mut storage_total = None;
    let mut storage_used = None;

    let disks = Disks::new_with_refreshed_list();

    for disk in &disks {
        if disk.mount_point() == Path::new("/") {
            storage_total = Some(disk.total_space());
            storage_used = Some(disk.total_space() - disk.available_space())
        }
    }

    Json(TartarusInfo {
        cpu_usage: sys.global_cpu_usage(),
        memory_total: sys.total_memory(),
        memory_used: sys.used_memory(),
        storage_total: storage_total.unwrap(),
        storage_used: storage_used.unwrap(),
        cpu_name: cpu_name.unwrap().to_string(),
        core_count: sys.cpus().len() as u64,
        os: System::long_os_version().unwrap(),
        kernel: System::kernel_version().unwrap(),
        hostname: System::host_name().unwrap(),
        uptime: System::uptime(),
    })
}

#[get("/tartarus_stats")]
pub async fn tartarus_stats(state: &rocket::State<SharedState>) -> Json<TartarusStats> {
    let state = state.read().await;
    let agents = state.agents.clone();
    let statistics = state.statistics.clone();

    Json(TartarusStats {
        registered_agents: agents.len() as u64,
        active_agents: agents.len() as u64, // TODO: fix
        packets_sent: statistics.packets_sent,
        packets_recv: statistics.packets_recv,
        average_response_latency: statistics.get_average_latency(), // safe
        total_traffic: statistics.get_total_traffic(),              //safe
        windows_agents: 0,                                          // TODO: fix
        linux_agents: agents.len() as u64,
    })
}

pub fn routes() -> Vec<rocket::Route> {
    rocket::routes![
        get_agents,
        list_agents,
        get_agent_history,
        tartarus_info,
        tartarus_stats,
    ]
}
