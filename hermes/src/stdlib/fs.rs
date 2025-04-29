use rhai::plugin::*;

use std::fs as std_fs;
use std::path::Path as std_Path;

#[export_module]
pub mod fs {
    use rhai::Array;
    use rhai::Dynamic;
    use std::io::Write;

    #[rhai_fn(return_raw)]
    pub fn read(file_path: &str) -> Result<String, Box<EvalAltResult>> {
        let path = std_Path::new(file_path);

        match std_fs::read_to_string(path) {
            Ok(file_contents) => Ok(file_contents),
            Err(err) => Err(err.to_string().into()),
        }
    }

    #[rhai_fn(return_raw)]
    pub fn read_lines(file_path: &str) -> Result<Vec<Dynamic>, Box<EvalAltResult>> {
        read(file_path).map(|x| x.split('\n').map(|x| x.to_string().into()).collect())
    }

    #[rhai_fn(return_raw)]
    pub fn write(file_path: &str, contents: &str) -> Result<(), Box<EvalAltResult>> {
        let path = std_Path::new(file_path);

        match std_fs::write(path, contents) {
            Ok(_) => Ok(()),
            Err(err) => Err(err.to_string().into()),
        }
    }

    #[rhai_fn(return_raw)]
    pub fn write_lines(file_path: &str, lines: Array) -> Result<(), Box<EvalAltResult>> {
        let mut filtered_lines = vec![];

        for line in lines {
            match line.try_cast_result::<String>() {
                Ok(line) => filtered_lines.push(line),
                Err(err) => return Err(format!("Expected string, got: {}", err.type_name()).into()),
            };
        }

        let lines = filtered_lines.join("\n") + "\n";
        write(file_path, &lines)
    }

    #[rhai_fn(return_raw)]
    pub fn append(file_path: &str, content: &str) -> Result<(), Box<EvalAltResult>> {
        let mut file = match std_fs::OpenOptions::new()
            .write(true)
            .append(true)
            .open(file_path)
        {
            Ok(file) => file,
            Err(err) => return Err(err.to_string().into()),
        };

        match file.write(content.as_bytes()) {
            Ok(_) => Ok(()),
            Err(err) => Err(err.to_string().into()),
        }
    }

    #[rhai_fn(return_raw)]
    pub fn append_lines(file_path: &str, lines: Array) -> Result<(), Box<EvalAltResult>> {
        let mut filtered_lines = vec![];

        for line in lines {
            match line.try_cast_result::<String>() {
                Ok(line) => filtered_lines.push(line),
                Err(err) => return Err(format!("Expected string, got: {}", err.type_name()).into()),
            };
        }

        let content = filtered_lines.join("\n") + "\n";
        append(file_path, &content)
    }

    #[rhai_fn(return_raw)]
    pub fn remove(file_path: &str) -> Result<(), Box<EvalAltResult>> {
        let path = std_Path::new(file_path);

        if path.is_file() {
            return match std::fs::remove_file(path) {
                Ok(_) => Ok(()),
                Err(err) => Err(err.to_string().into()),
            };
        }

        if path.is_dir() {
            return match std::fs::remove_dir_all(path) {
                Ok(_) => Ok(()),
                Err(err) => Err(err.to_string().into()),
            };
        }

        Err("Provided path is neither a file nor a directory".into())
    }

    #[rhai_fn(return_raw)]
    pub fn create(file_path: &str) -> Result<(), Box<EvalAltResult>> {
        let path = std_Path::new(file_path);

        let result = match path.parent() {
            Some(parent) => mkdir(parent.to_str().unwrap()),
            None => Ok(()),
        };

        match result {
            Ok(_) => {}
            Err(err) => return Err(err.to_string().into()),
        };

        match std_fs::File::create(file_path) {
            Ok(_) => Ok(()),
            Err(err) => Err(err.to_string().into()),
        }
    }

    #[rhai_fn(return_raw)]
    pub fn mkdir(dir_path: &str) -> Result<(), Box<EvalAltResult>> {
        let path = std_Path::new(dir_path);

        match std_fs::create_dir_all(path) {
            Ok(_) => Ok(()),
            Err(err) => Err(err.to_string().into()),
        }
    }

    pub fn exists(path: &str) -> bool {
        std_Path::new(path).exists()
    }

    pub fn is_file(path: &str) -> bool {
        std_Path::new(path).is_file()
    }

    pub fn is_dir(path: &str) -> bool {
        std_Path::new(path).is_dir()
    }
}
