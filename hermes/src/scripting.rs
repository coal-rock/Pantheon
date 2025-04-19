use std::sync::Arc;
use tokio::sync::RwLock;

use anyhow::Result;
use rhai::{plugin::*, Scope};

use crate::state::State;

pub struct ScriptingEngine {
    engine: Engine,
    scope: Scope<'static>,
}

impl ScriptingEngine {
    pub async fn new(state: Arc<RwLock<State>>) -> ScriptingEngine {
        let mut engine = Engine::new();

        let agent = exported_module!(agent);
        engine.register_static_module("agent", agent.into());

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

#[export_module]
mod agent {
    pub const MY_NUMBER: i64 = 42;

    pub fn greet(name: &str) -> String {
        format!("hello, {}!", name)
    }
}
