use std::env;
use std::fs;
use std::path::Path;

fn rename(pattern: String, new_name: String, path: &Path) {
    let mut i: u32 = 0;
    for entry in fs::read_dir(path).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        if path.is_file() {
            let file_name = path.file_name().unwrap().to_str().unwrap();
            if file_name.starts_with(&pattern) {
                let extension = path.extension().unwrap().to_str().unwrap();
                let new_file_name = format!("{new_name}_{i}.{}", extension);
                let new_path = path.with_file_name(new_file_name);
                fs::rename(&path, &new_path).expect("Unable to rename file");
                i += 1;
            }
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let pattern = args[1].clone();
    let new_name = args[2].clone();
    let path = Path::new(".");
    rename(pattern, new_name, path);
}
