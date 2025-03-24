use crate::SharedState;

use rocket::http::Status;
use rocket::outcome::try_outcome;
use rocket::request::{self, FromRequest, Request};
use rocket::State;

pub struct Auth;

#[rocket::async_trait]
impl<'r> FromRequest<'r> for Auth {
    type Error = ();

    async fn from_request(req: &'r Request<'_>) -> request::Outcome<Self, ()> {
        let state = try_outcome!(req.guard::<&State<SharedState>>().await);

        let config = &state.read().await.config;

        // No token specified means we are vacuously authenticated
        let expected_token = match &config.token {
            Some(str) => str,
            None => return rocket::outcome::Outcome::Success(Auth),
        };

        let given_token = match req.headers().get_one("Authorization") {
            Some(auth) => auth,
            None => return rocket::outcome::Outcome::Error((Status::Unauthorized, ())),
        };

        match expected_token == given_token {
            true => rocket::outcome::Outcome::Success(Auth),
            false => rocket::outcome::Outcome::Error((Status::Unauthorized, ())),
        }
    }
}
