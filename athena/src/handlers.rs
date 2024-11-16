use actix_web::{get, web, HttpResponse, Responder};
use reqwest;
use serde_json::json;
use tera::Tera;
use crate::models::AgentStatus;

#[get("/agents")]
pub async fn get_agents(tera: web::Data<Tera>) -> impl Responder {
    let client = reqwest::Client::new();
    let response = client
        .get("http://127.0.0.1:5000/api/agents") // Updated URL
        .send()
        .await;

    match response {
        Ok(resp) => match resp.json::<Vec<AgentStatus>>().await {
            Ok(agents) => {
                let mut context = tera::Context::new();
                context.insert("agents", &agents);
                let rendered = tera.render("agents.html", &context);

                match rendered {
                    Ok(html) => HttpResponse::Ok().content_type("text/html").body(html),
                    Err(err) => {
                        eprintln!("Template rendering error: {:?}", err);
                        HttpResponse::InternalServerError().finish()
                    }
                }
            }
            Err(err) => {
                eprintln!("Error deserializing agents: {:?}", err);
                HttpResponse::InternalServerError().finish()
            }
        },
        Err(err) => {
            eprintln!("Error fetching agents: {:?}", err);
            HttpResponse::InternalServerError().finish()
        }
    }
}
