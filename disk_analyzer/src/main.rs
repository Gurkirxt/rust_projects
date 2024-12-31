use crossterm::{cursor, terminal, ExecutableCommand};
use std::env;
use std::io::{stdout, Write};
use std::path::Path;
use walkdir::WalkDir;

fn terminal_width() -> Result<u16, Box<dyn std::error::Error>> {
    let (width, _) = terminal::size()?;
    Ok(width)
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("Usage: {} <path>", args[0]);
        std::process::exit(1);
    }

    let path = &args[1];
    let path = Path::new(path);

    if !path.exists() {
        eprintln!("The provided path does not exist: {}", path.display());
        std::process::exit(1);
    }

    println!("Inspecting directory: {}", path.display());

    match inspect_directory(path) {
        Ok(_) => {}
        Err(e) => eprintln!("Error: {}", e),
    }
}

fn inspect_directory(path: &Path) -> Result<(), Box<dyn std::error::Error>> {
    let mut total_size = 0;
    let mut file_count = 0;
    let mut dir_count = 0;

    println!("\nSummary for directory:");

    for entry in WalkDir::new(path) {
        let entry = entry?;
        let metadata = entry.metadata()?;

        if metadata.is_file() {
            file_count += 1;
            total_size += metadata.len();
        } else if metadata.is_dir() {
            dir_count += 1;
        }
    }

    println!(
        "Total size: {} bytes ({})",
        total_size,
        readable_size(total_size)
    );
    println!("Number of files: {}", file_count);
    println!("Number of subdirectories: {}", dir_count);

    println!("\nIndividual entries:");
    let mut stdout = stdout();
    for entry in WalkDir::new(path) {
        let entry = entry?;
        let metadata = entry.metadata()?;
        let entry_size = if metadata.is_file() {
            metadata.len()
        } else {
            0
        };

        let relative_path = entry.path().strip_prefix(path).unwrap_or(entry.path());
        let human_size = readable_size(entry_size);

        let terminal_width = terminal_width()? as usize;
        let path_column_width = terminal_width.saturating_sub(15);

        stdout.execute(cursor::MoveToColumn(0))?;
        write!(
            stdout,
            "{:<path_column_width$}",
            relative_path.display(),
            path_column_width = path_column_width
        )?;
        stdout.execute(cursor::MoveToColumn(terminal_width as u16 - 10))?;
        writeln!(stdout, "{}", human_size)?;
    }

    Ok(())
}

fn readable_size(size: u64) -> String {
    let units = ["B", "KB", "MB", "GB", "TB"];
    let mut size = size as f64;
    let mut unit = 0;

    while size >= 1024.0 && unit < units.len() - 1 {
        size /= 1024.0;
        unit += 1;
    }

    format!("{:.2} {}", size, units[unit])
}
