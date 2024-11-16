use actix_web::{App, HttpServer, web};
use tera::Tera;

mod handlers;
mod models;
mod routes;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let tera = Tera::new("templates/**/*").expect("Failed to initialize Tera templates");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(tera.clone())) // Updated to app_data
            .configure(routes::configure_routes)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
