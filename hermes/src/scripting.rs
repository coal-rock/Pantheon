use std::sync::Arc;
use tokio::sync::RwLock;

use anyhow::Result;
use rhai::{plugin::*, Scope};

use crate::state::State;
use crate::stdlib::*;

pub struct ScriptingEngine {
    engine: Engine,
    scope: Scope<'static>,
}

impl ScriptingEngine {
    pub async fn new(state: Arc<RwLock<State>>) -> ScriptingEngine {
        let mut engine = Engine::new();

        let mut modules = vec![];

        {
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

        let mut scope = Scope::new();
        {
            let state = state.read().await;
            scope.push_constant("AGENT_ID", state.get_agent_id());
        }

        ScriptingEngine { engine, scope }
    }

    pub async fn execute(&mut self, script: &str) -> Result<(), Box<EvalAltResult>> {
        let ast = self.engine.compile(script)?;

        self.engine.run_ast_with_scope(&mut self.scope, &ast)?;
        Ok(())
    }
}
