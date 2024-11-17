use std::env;
use std::fs;
use std::path::Path;

fn organize(path: &Path) {
    for entry in fs::read_dir(path).unwrap() {
        let entry = entry.unwrap();
        let src = entry.path();
        if src.is_file() {
            if let Some(extension) = src.extension().and_then(|ext| ext.to_str()) {
                let dest_dir = src.parent().unwrap().join(extension);
                if !dest_dir.is_dir() {
                    fs::create_dir_all(&dest_dir).expect("unable to create directory");
                }
                let dest = dest_dir.join(src.file_name().unwrap());
                fs::rename(src, dest).expect("Unable to move file");
            }
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let dir = args[1].clone();
    let path = Path::new(&dir);
    organize(path);
}
