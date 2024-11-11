#[macro_use]
extern crate rocket;

mod admin;
mod agent;

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/agent", agent::routes())
        .mount("/admin", admin::routes())
}
