use actix_web::web;

use crate::handlers::get_agents;

/// Configures all the application routes
pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("") // Use scope to group routes
            .service(get_agents) // Register the /agents route
    );
}
