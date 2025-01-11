use regex::Regex;
use std::env;
use std::fs;
use std::fs::read_to_string;
use std::fs::File;
// use std::fs::Permissions;
use std::io::{self, BufRead, Write};
// use std::os::unix::fs::PermissionsExt;
use std::path::Path;
use std::process::Command;
// use tempfile::NamedTempFile;
use walkdir::WalkDir;

fn grep(dir: String, pattern: String) {
    let path = Path::new(&dir);
    let re = Regex::new(&pattern).unwrap();

    for entry in WalkDir::new(path) {
        match entry {
            Ok(entry) => {
                let path = entry.path();

                if !path.is_file() {
                    continue;
                }

                let file_content = match read_to_string(path) {
                    Ok(content) => content,
                    Err(_e) => {
                        continue;
                    }
                };

                for (line_number, line) in file_content.lines().enumerate() {
                    if re.is_match(line) {
                        println!("{} (line {}): {}", path.display(), line_number + 1, line);
                    }
                }
            }
            Err(e) => eprintln!("Error reading entry: {}", e),
        }
    }
}
fn find(dir: String, pattern: String) {
    let path = Path::new(&dir);
    let re = Regex::new(pattern.as_str()).unwrap();
    for entry in WalkDir::new(path) {
        match entry {
            Ok(entry) => {
                let path = entry.path();
                let path_str = path.to_str().unwrap();
                if re.is_match(path_str) {
                    println!("{}", path_str);
                }
            }
            Err(e) => eprintln!("Error reading entry: {}", e),
        }
    }
}

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
                fs::rename(&path, &new_path).expect("unable to rename file");
                i += 1;
            }
        }
    }
}

fn crename(dir: String, editor: String, user_file_path: String) -> io::Result<()> {
    let path = Path::new(&dir);

    // Collect all filenames with extensions in the directory
    let mut file_paths = Vec::new(); // To store the original file paths
    let mut file_names = Vec::new(); // To store only the filenames

    for entry in WalkDir::new(path) {
        match entry {
            Ok(entry) => {
                let path = entry.path();

                if !path.is_file() {
                    continue; // Skip directories or non-files
                }

                if let Some(file_name) = path.file_name().and_then(|n| n.to_str()) {
                    file_paths.push(path.to_path_buf());
                    file_names.push(file_name.to_string());
                }
            }
            Err(e) => eprintln!("Error reading entry: {}", e),
        }
    }

    // Write the filenames to the user-provided file
    let mut user_file = File::open(&user_file_path)?;
    // let permissions = Permissions::from_mode(0o644); // Read and write for owner, read-only for others
    // user_file.set_permissions(permissions)?;
    for file_name in &file_names {
        writeln!(user_file, "{}", file_name)?;
    }

    // Open the user-provided file in the text editor
    if let Err(e) = Command::new(editor.as_str())
        .arg(user_file_path.as_str())
        .status()
    {
        eprintln!("Failed to open the file in the editor: {}", e);
        return Err(e);
    }

    // Read the modified file names back into a list
    let edited_file = File::open(&user_file_path)?;
    let new_file_names: Vec<String> = io::BufReader::new(edited_file)
        .lines()
        .filter_map(|line| {
            match line {
                Ok(line) => {
                    // Trim whitespace and ignore empty lines
                    let trimmed_line = line.trim();
                    if !trimmed_line.is_empty() {
                        Some(trimmed_line.to_string())
                    } else {
                        None
                    }
                }
                Err(e) => {
                    eprintln!("Error reading line: {}", e);
                    None
                }
            }
        })
        .collect();

    // Check if all files are accounted for
    if file_names.len() != new_file_names.len() {
        eprintln!("Error: The number of filenames in the edited file does not match the original.");
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "Mismatched filename count",
        ));
    }

    // Rename the files
    for (old_path, new_name) in file_paths.iter().zip(new_file_names.iter()) {
        let new_path = old_path.parent().unwrap().join(new_name);
        if let Err(e) = fs::rename(old_path, &new_path) {
            eprintln!("Failed to rename {:?} to {:?}: {}", old_path, new_path, e);
        } else {
            println!("Renamed {:?} to {:?}", old_path, new_path);
        }
    }

    Ok(())
}

fn organize(dir: String) {
    let path = Path::new(&dir);
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
                fs::rename(src, dest).expect("unable to move file");
            }
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let opt = args[1].clone();
    if opt == "rename" {
        let path_str = args[2].clone();
        let pattern = args[3].clone();
        let new_name = args[4].clone();
        let path = Path::new(&path_str);
        rename(pattern, new_name, path);
    } else if opt == "organize" {
        let dir = args[2].clone();
        organize(dir);
    } else if opt == "find" {
        let dir = args[2].clone();
        let pattern = args[3].clone();
        find(dir, pattern);
    } else if opt == "grep" {
        let dir = args[2].clone();
        let pattern = args[3].clone();
        grep(dir, pattern);
    } else if opt == "crename" {
        let dir = args[2].clone();
        let editor = if args.len() > 2 {
            args[2].clone()
        } else {
            // Default to the system's default editor
            env::var("EDITOR").unwrap_or_else(|_| "xdg-open".to_string())
        };
        let temp_file = args[3].clone();
        match crename(dir, editor, temp_file) {
            Ok(()) => println!("Files renamed successfully!"),
            Err(e) => eprintln!("Failed to rename files: {}", e),
        }
    } else {
        println!("invalid arguments")
    }
}
