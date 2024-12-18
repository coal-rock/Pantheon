use rocket::fs::NamedFile;
use std::path::Path;

/// Serves a compiled file from the specified directory
#[get("/compiled/<filename>")]
pub async fn serve_compiled_file(filename: &str) -> Option<NamedFile> {
    // Change this to your compiled files directory
    let compiled_dir = Path::new("target/release/");
    let file_path = compiled_dir.join(filename);

    // Attempt to serve the file
    NamedFile::open(file_path).await.ok()
}