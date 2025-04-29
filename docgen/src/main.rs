use std::io::Write;

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
        .format_sections_with(rhai_autodocs::export::SectionFormat::Tabs)
        .export(&engine)
        .expect("failed to generate docs");

    for (name, doc) in rhai_autodocs::generate::mdbook().generate(&docs).unwrap() {
        println!("generating documentation for: {}", name);

        std::fs::write(
            std::path::PathBuf::from_iter([path, &format!("{}.md", &name)]),
            doc,
        )
        .expect("failed to write docs");

        // [fs](./fs.md)
        writeln!(summary_file, "[{}](./{}.md)", name, name)
            .expect("unable to write to SUMMARY file");
    }

    println!("documentation generated to {path:?}");
}
