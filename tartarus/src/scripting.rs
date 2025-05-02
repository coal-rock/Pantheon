use anyhow::anyhow;
use anyhow::Result;
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
    pub fn from_str(input: &str) -> Result<Script> {
        let input = input.trim();

        let lines = input.split('\n').collect::<Vec<&str>>();

        let line = match lines.get(0) {
            Some(line) => line,
            None => return Err(anyhow!("Script cannot be empty")),
        };

        if *line != "---" {
            return Err(anyhow!("Script must start with metadata"));
        }

        let mut index = 1;

        let header_end = loop {
            let line = match lines.get(index) {
                Some(line) => line,
                None => return Err(anyhow!("Script metadata is never closed")),
            };

            if *line == "---" {
                break index;
            }

            index += 1;
        };

        if header_end == 1 {
            return Err(anyhow!("Script metadata cannot be empty"));
        }

        if lines.len() < header_end + 1 {
            return Err(anyhow!("Script must contain body"));
        }

        let header = lines[1..header_end].join("\n");

        let mut script = match toml::from_str::<Script>(&header) {
            Ok(script) => script,
            Err(err) => return Err(anyhow!(err.to_string())),
        };

        script.body = lines[header_end + 1..].join("\n");

        Ok(script)
    }
}

#[derive(Deserialize, Debug)]
pub struct Param {
    name: String,
    arg_name: String,
    description: String,
    r#type: ParamType,
    placeholder: String,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum ParamType {
    String,
    Number,
    Float,
    Bool,
    #[allow(dead_code)]
    Array(Vec<ParamType>),
}
