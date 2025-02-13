use rocket::fs::NamedFile;

use crate::SharedState;

#[get("/linux")]
pub async fn get_linux_binary(state: &rocket::State<SharedState>) -> Option<NamedFile> {
    let config = state.read().await.config.clone();
    NamedFile::open(config.binary_path.join("linux")).await.ok()
}

#[get("/windows")]
pub async fn get_windows_binary(state: &rocket::State<SharedState>) -> Option<NamedFile> {
    let config = state.read().await.config.clone();
    NamedFile::open(config.binary_path.join("windows"))
        .await
        .ok()
}

pub fn routes() -> Vec<rocket::Route> {
    rocket::routes![get_linux_binary, get_windows_binary]
}
