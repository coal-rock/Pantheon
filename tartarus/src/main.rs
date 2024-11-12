#[macro_use]
extern crate rocket;

mod admin;
mod agent;

// #[rocket::main]
// async fn main() {
//     // Start the network server in the background
//     tokio::spawn(async {
//         start_server("0.0.0.0:8081").await.unwrap();
//     });
//
//     // Start the admin server
//     rocket().await.unwrap();
// }
//
// #[launch]
// async fn rocket() -> _ {
//     rocket::build().mount("/admin", routes())
// }

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/admin", admin::routes())
        .mount("/agent", agent::routes())
}
