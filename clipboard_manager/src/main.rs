use arboard::Clipboard;
use directories::ProjectDirs;
use serde::{Deserialize, Serialize};
use std::{
    collections::VecDeque,
    fs,
    path::PathBuf,
    thread,
    time::{Duration, SystemTime},
};

const MAX_HISTORY: usize = 10;

#[derive(Debug, Serialize, Deserialize)]
struct ClipboardEntry {
    content: String,
    timestamp: SystemTime,
}

struct ClipboardManager {
    clipboard: Clipboard,
    history: VecDeque<ClipboardEntry>,
    save_path: PathBuf,
}

impl ClipboardManager {
    fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let proj_dirs = ProjectDirs::from("com", "clipboard", "manager")
            .expect("Failed to get project directories");

        let save_dir = proj_dirs.data_dir();
        fs::create_dir_all(save_dir)?;
        let save_path = save_dir.join("history.json");

        let history = if save_path.exists() {
            let data = fs::read_to_string(&save_path)?;
            serde_json::from_str(&data)?
        } else {
            VecDeque::with_capacity(MAX_HISTORY)
        };

        Ok(Self {
            clipboard: Clipboard::new()?,
            history,
            save_path,
        })
    }

    fn save_history(&self) -> Result<(), Box<dyn std::error::Error>> {
        let data = serde_json::to_string(&self.history)?;
        fs::write(&self.save_path, data)?;
        Ok(())
    }

    fn add_entry(&mut self, content: String) {
        if self.history.is_empty() || self.history.front().unwrap().content != content {
            self.history.push_front(ClipboardEntry {
                content,
                timestamp: SystemTime::now(),
            });

            if self.history.len() > MAX_HISTORY {
                self.history.pop_back();
            }

            if let Err(e) = self.save_history() {
                eprintln!("Failed to save history: {}", e);
            }
        }
    }

    fn monitor_clipboard(&mut self) {
        let mut last_content = String::new();

        loop {
            if let Ok(content) = self.clipboard.get_text() {
                if !content.is_empty() && content != last_content {
                    self.add_entry(content.clone());
                    last_content = content;
                }
            }
            thread::sleep(Duration::from_millis(500));
        }
    }

    fn print_history(&self) {
        println!("\nClipboard History:");
        println!("-----------------");
        for (i, entry) in self.history.iter().enumerate() {
            println!("{}. {}", i + 1, entry.content);
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut manager = ClipboardManager::new()?;

    println!("Clipboard Manager Started");
    println!("Press Ctrl+C to exit");

    // Print initial history
    manager.print_history();

    // Start monitoring clipboard
    manager.monitor_clipboard();

    Ok(())
}
