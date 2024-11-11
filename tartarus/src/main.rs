#[macro_use] extern crate rocket;

mod routes;
use routes::*;
use talaria::server::start_server;

#[rocket::main]
async fn main() {
    // Start the network server in the background
    tokio::spawn(async {
        start_server("0.0.0.0:8081").await.unwrap();
    });

    // Start the admin server
    rocket().await.unwrap();
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/admin", routes())
}
