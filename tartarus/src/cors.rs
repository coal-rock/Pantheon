use rocket::{
    fairing::{Fairing, Info, Kind},
    http::Header,
    Request, Response, State,
};

use crate::SharedState;

pub struct CORS;

#[rocket::async_trait]
impl Fairing for CORS {
    fn info(&self) -> Info {
        Info {
            name: "CORS Fairing",
            kind: Kind::Response,
        }
    }

    async fn on_response<'r>(&self, req: &'r Request<'_>, response: &mut Response<'r>) {
        let state = req.guard::<&State<SharedState>>().await.unwrap();
        let config = &state.read().await.config;

        response.set_header(Header::new(
            "Access-Control-Allow-Origin",
            config.cors.clone(),
        ));
        response.set_header(Header::new(
            "Access-Control-Allow-Methods",
            "GET, POST, PUT, DELETE, OPTIONS",
        ));
        response.set_header(Header::new(
            "Access-Control-Allow-Headers",
            "Content-Type, Authorization",
        ));
    }
}
