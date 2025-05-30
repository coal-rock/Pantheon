use rhai::plugin::*;

use rhai::export_module;
use rhai::Module;

#[export_module]
pub mod http {
    // use crate::stdlib::error::error::ScriptError;

    use reqwest::header::{HeaderMap, HeaderValue};
    use reqwest::StatusCode;

    #[rhai_fn(return_raw)]
    pub fn post(
        url: &str,
        body: &str,
        header_name: Option<&str>,
        header_value: Option<&str>,
    ) -> Result<String, Box<EvalAltResult>> {
        let client = reqwest::blocking::Client::new();

        let response = client
            .post(url)
            .header(
                match header_name {
                    Some(hn) => hn,
                    None => "Content-type",
                },
                match header_value {
                    Some(hv) => hv,
                    None => "application/json",
                },
            )
            .body(body.to_owned())
            .send();

        let response = match response {
            Ok(ok) => ok,
            // TODO: find what causes this to error
            // Error when sending the request
            Err(_err) => return Err("".to_string().into()),
        };

        // If not 200 status code, return some error?
        // but what if other 200 status code?
        match response.status() {
            StatusCode::OK => (),
            // TODO: find what causes this to error
            // Send request, but got an error
            _ => return Err("".to_string().into()),
        };

        let text = response.text();

        match text {
            Ok(ok) => Ok(ok),
            // TODO: find what causes this to error
            // Unsure.. Maybe some general post error?
            Err(_err) => Err("".to_string().into()),
        }
    }

    #[rhai_fn(return_raw)]
    pub fn get(
        url: &str,
        _params: Vec<(&str, &str)>, // param of what exactly?
        // this &'static str is the only way I can create this HeaderMap
        // Seems grossly incorrect
        headers: Option<Vec<(&'static str, &str)>>,
    ) -> Result<String, Box<EvalAltResult>> {
        let mut header = HeaderMap::new();

        match headers {
            Some(h) => {
                h.iter().for_each(|i| {
                    header.insert(i.0, HeaderValue::from_str(i.1).unwrap());
                });
                ()
            }
            None => {
                header.insert("Content-type", HeaderValue::from_static("application/json"));
                ()
            }
        };

        let client = reqwest::blocking::Client::new();

        let response = match client.get(url).headers(header).send() {
            Ok(ok) => ok,
            // TODO: find what causes this to error
            Err(_err) => return Err("".to_string().into()),
        };

        match response.status() {
            StatusCode::OK => (),
            // TODO: find what causes this to error
            _ => return Err("".to_string().into()),
        };

        match response.text() {
            Ok(ok) => Ok(ok),
            // TODO: find what causes this to error
            Err(_err) => Err("".to_string().into()),
        }
    }

    #[rhai_fn(return_raw)]
    pub fn put(
        url: &str,
        _data: &str,
        body: &str,
        headers: Option<Vec<(&'static str, &str)>>,
    ) -> Result<String, Box<EvalAltResult>> {
        let mut header = HeaderMap::new();

        match headers {
            Some(h) => {
                h.iter().for_each(|i| {
                    header.insert(i.0, HeaderValue::from_str(i.1).unwrap());
                });
                ()
            }
            None => {
                header.insert("Content-type", HeaderValue::from_static("application/json"));
                ()
            }
        };

        let client = reqwest::blocking::Client::new();

        let response = match client.put(url).headers(header).body(body.to_owned()).send() {
            Ok(response_body) => response_body,
            Err(_err) => return Err("".to_string().into()),
        };

        match response.status() {
            StatusCode::OK => (),
            // TODO: find what causes this to error
            _ => return Err("".to_string().into()),
        };

        match response.text() {
            Ok(ok) => Ok(ok),
            // TODO: find what causes this to error
            Err(_err) => Err("".to_string().into()),
        }
    }

    // head of what?
    #[rhai_fn(return_raw)]
    pub fn head() -> Result<String, Box<EvalAltResult>> {
        unimplemented!()
    }

    #[rhai_fn(return_raw)]
    pub fn delete(url: &str) -> Result<String, Box<EvalAltResult>> {
        let client = reqwest::blocking::Client::new();

        let response = match client.delete(url).send() {
            Ok(ok) => ok,
            // TODO: find what causes this to error
            Err(_err) => return Err("".to_string().into()),
        };

        match response.text() {
            Ok(ok) => Ok(ok),
            // TODO: find what causes this to error
            Err(_err) => Err("".to_string().into()),
        }
    }

    #[rhai_fn(return_raw)]
    pub fn download(
        url: &str,
        params: Vec<(&str, &str)>,
        headers: Option<Vec<(&'static str, &str)>>,
        output_path: &str,
    ) -> Result<String, Box<EvalAltResult>> {
        let response_body = match get(url, params, headers) {
            Ok(ok) => ok,
            // TODO: find what causes this to error
            Err(_err) => return Err("".to_string().into()),
        };

        let mut out = match std::fs::File::create(output_path) {
            Ok(ok) => ok,
            // TODO: find what causes this to error
            Err(_err) => return Err("".to_string().into()),
        };

        match std::io::copy(&mut response_body.as_bytes(), &mut out) {
            Ok(bytes_writted) => Ok(bytes_writted.to_string()),
            Err(_err) => Err("".to_string().into()),
        }
    }

    #[rhai_fn(return_raw)]
    pub fn upload(url: &str, file_path: &str) -> Result<String, Box<EvalAltResult>> {
        let file = match std::fs::File::open(file_path) {
            Ok(ok) => ok,
            Err(_err) => return Err("".to_string().into()),
        };

        let client = reqwest::blocking::Client::new();
        let res = client.post(url).body(file).send();

        match res {
            Ok(ok) => Ok(ok.text().unwrap()),
            Err(_err) => Err("".to_string().into()),
        }
    }
}
