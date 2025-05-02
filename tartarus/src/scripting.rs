use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Script {
    name: String,
    description: String,
    params: Vec<Param>,
    #[serde(skip)]
    body: String,
}

impl Script {
    pub fn from_str(input: &str) -> Script {
        toml::from_str(input).unwrap()
    }
}

#[derive(Deserialize, Debug)]
pub struct Param {
    name: String,
    description: String,
    r#type: ParamType,
    placeholder: String,
}

#[derive(Deserialize, Debug)]
pub enum ParamType {
    String,
    Number,
    Float,
    Bool,
    #[allow(dead_code)]
    Array(Vec<ParamType>),
}
