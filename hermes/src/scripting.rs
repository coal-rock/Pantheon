use anyhow::Result;
use rhai::{plugin::*, Scope};

pub struct ScriptingEngine {
    engine: Engine,
}

impl ScriptingEngine {
    pub fn new() {
        let mut engine = Engine::new();
        let module = exported_module!(hermes);
        engine.register_static_module("hermes", module.into());
    }

    async fn eval(&mut self, script: &str) -> Result<(), Box<EvalAltResult>> {
        let ast = self.engine.compile(script)?;
        let mut scope = Scope::new();
        scope.push("hello_world", 1337);

        self.engine.run_ast_with_scope(&mut scope, &ast)?;
        Ok(())
    }
}

#[export_module]
mod hermes {
    pub const MY_NUMBER: i64 = 42;

    pub fn greet(name: &str) -> String {
        format!("hello, {}!", name)
    }
}
