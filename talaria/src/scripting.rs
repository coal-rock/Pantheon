use anyhow::{anyhow, Result};
use bincode::{Decode, Encode};
use rhai::{plugin::*, Scope};
use serde::{Deserialize, Serialize};

use crate::stdlib::*;

pub struct ScriptingEngine {
    engine: Engine,
    scope: Scope<'static>,
}

impl ScriptingEngine {
    pub fn new() -> ScriptingEngine {
        let mut engine = Engine::new();

        let mut modules = vec![];

        {
            modules.push(("error", exported_module!(error::error)));
            modules.push(("crypto", exported_module!(crypto::crypto)));
            modules.push(("env", exported_module!(env::env)));
            modules.push(("fs", exported_module!(fs::fs)));
            modules.push(("http", exported_module!(http::http)));
            modules.push(("net", exported_module!(net::net)));
            modules.push(("proc", exported_module!(proc::proc)));
            modules.push(("sys", exported_module!(sys::sys)));
            modules.push(("time", exported_module!(time::time)));
        }

        for (module_name, module) in modules {
            engine.register_static_module(module_name, module.into());
        }

        let scope = Scope::new();
        ScriptingEngine { engine, scope }
    }

    pub async fn execute(&mut self, script: &str) -> Result<(), Box<EvalAltResult>> {
        let ast = self.engine.compile(script)?;

        self.engine.run_ast_with_scope(&mut self.scope, &ast)?;
        Ok(())
    }

    pub fn get_engine(&self) -> &Engine {
        &self.engine
    }
}

#[derive(Clone, Deserialize, Serialize, Encode, Decode, Debug)]
pub struct Script {
    pub name: String,
    pub description: String,
    pub params: Vec<Param>,
    #[serde(skip)]
    pub body: String,
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

#[derive(Clone, Debug, Deserialize, Serialize, Encode, Decode)]
pub struct Param {
    pub name: String,
    pub arg_name: String,
    pub description: String,
    pub r#type: ParamType,
    pub placeholder: String,
}

#[derive(Clone, Deserialize, Serialize, Encode, Decode, Debug)]
#[serde(rename_all = "lowercase")]
pub enum ParamType {
    String,
    Number,
    Float,
    Bool,
    Array,
}
