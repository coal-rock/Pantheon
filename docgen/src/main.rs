use handlebars::Handlebars;
use rhai_autodocs::item::Item;
use serde_json::json;
use std::io::Write;
use std::path::Path;
use talaria::scripting::ScriptingEngine;

fn main() {
    let path = "./docs";

    let engine = ScriptingEngine::new();
    let engine = engine.get_engine();

    let _ = std::fs::create_dir(path);
    std::fs::write(format!("{}/SUMMARY.md", path), "").expect("unable to write to SUMMARY file");

    let mut summary_file = std::fs::OpenOptions::new()
        .write(true)
        .append(true)
        .open(format!("{}/SUMMARY.md", path))
        .expect("unable to open SUMMARY file");

    let docs = rhai_autodocs::export::options()
        .include_standard_packages(true)
        .order_items_with(rhai_autodocs::export::ItemsOrder::Alphabetical)
        .format_sections_with(rhai_autodocs::export::SectionFormat::Rust)
        .export(&engine)
        .expect("failed to generate docs");

    let mut handlebars = Handlebars::new();

    let _ = handlebars
        .register_template_file("module", Path::new("module.hbs"))
        .expect("unable to load template");

    // hack
    let _ = handlebars
        .register_partial("ContentPartial", "{{{content}}}")
        .expect("partial is valid");

    for module in docs.sub_modules {
        println!("generating documentation for: {}", module.namespace);

        let data = json!({
            "title": module.name,
            "description": module.documentation,
            "namespace": module.namespace,
            "items": module.items,
        });

        let doc = handlebars
            .render("module", &data)
            .expect("unable to render template");

        std::fs::write(
            std::path::PathBuf::from_iter([path, &format!("{}.md", &module.name)]),
            doc,
        )
        .expect("failed to write docs");

        // [fs](./fs.md)
        writeln!(summary_file, "[{}](./{}.md)", module.name, module.name)
            .expect("unable to write to SUMMARY file");
    }

    println!("docs written to: {}", path);
}
