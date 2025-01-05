use std::fs::File;
use std::io::BufReader;
use std::path::{Path, PathBuf};
use std::sync::{
    atomic::{AtomicUsize, Ordering},
    Arc,
};
use std::time::Duration;

use anyhow::{Context, Result};
use clap::Parser;
use crossterm::{
    cursor,
    event::{self, Event, KeyCode},
    terminal::{disable_raw_mode, enable_raw_mode},
};
use rodio::{Decoder, OutputStream, Sink};
use walkdir::WalkDir;

#[derive(Parser)]
#[command(name = "musicli")]
#[command(about = "A CLI music player that finds and plays music in directories")]
struct Cli {
    /// Directory containing music files (defaults to current directory)
    #[arg(value_name = "DIR")]
    dir: Option<PathBuf>,

    /// Volume level (0.0 to 1.0)
    #[arg(short, long, default_value = "1.0")]
    volume: f32,

    /// Search recursively in subdirectories
    #[arg(short, long)]
    recursive: bool,
}

struct Player {
    sink: Sink,
    current_track: Arc<AtomicUsize>,
    playlist: Vec<PathBuf>,
    volume: f32,
}

fn is_audio_file(path: &Path) -> bool {
    if let Some(mime) = mime_guess::from_path(path).first() {
        return mime.type_() == "audio";
    }
    false
}

fn find_music_files(dir: &Path, recursive: bool) -> Vec<PathBuf> {
    let mut files = Vec::new();
    let walker = if recursive {
        WalkDir::new(dir)
    } else {
        WalkDir::new(dir).max_depth(1)
    };

    for entry in walker.into_iter().filter_map(|e| e.ok()) {
        let path = entry.path();
        if path.is_file() && is_audio_file(path) {
            files.push(path.to_path_buf());
        }
    }

    // Sort files by name for consistent ordering
    files.sort();
    files
}

impl Player {
    fn new(
        stream_handle: &rodio::OutputStreamHandle,
        playlist: Vec<PathBuf>,
        volume: f32,
    ) -> Result<Self> {
        let sink = Sink::try_new(stream_handle).context("Failed to create audio sink")?;
        sink.set_volume(volume);

        Ok(Self {
            sink,
            current_track: Arc::new(AtomicUsize::new(0)),
            playlist,
            volume,
        })
    }

    fn play_current(&self) -> Result<()> {
        let current = self.current_track.load(Ordering::SeqCst);
        if let Some(path) = self.playlist.get(current) {
            self.sink.stop();
            self.sink.set_volume(self.volume);

            println!("\nFile: {}", path.display());
            println!("Volume: {}", self.volume);
            self.print_controls();

            let file = BufReader::new(File::open(path).context("Failed to open audio file")?);
            let source = Decoder::new(file).context("Failed to decode audio file")?;
            self.sink.append(source);
        }
        Ok(())
    }

    fn next_track(&self) -> Result<()> {
        let current = self.current_track.load(Ordering::SeqCst);
        if current + 1 < self.playlist.len() {
            self.current_track.store(current + 1, Ordering::SeqCst);
            self.play_current()?;
        }
        Ok(())
    }

    fn previous_track(&self) -> Result<()> {
        let current = self.current_track.load(Ordering::SeqCst);
        if current > 0 {
            self.current_track.store(current - 1, Ordering::SeqCst);
            self.play_current()?;
        }
        Ok(())
    }

    fn print_controls(&self) {
        println!("\nControls:\nn - Next track\np - Previous track\nq - Quit");
        execute!(stdout(), cursor::MoveToColumn(0))?;
    }
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    // Use current directory if none specified
    let dir = cli.dir.unwrap_or_else(|| PathBuf::from("."));

    if !dir.exists() {
        anyhow::bail!("Directory not found: {}", dir.display());
    }

    // Find all music files in the directory
    println!("Searching for music files in: {}", dir.display());
    if cli.recursive {
        println!("(including subdirectories)");
    }

    let playlist = find_music_files(&dir, cli.recursive);

    if playlist.is_empty() {
        anyhow::bail!("No music files found in the specified directory");
    }

    println!("Found {} music files", playlist.len());

    // Set up audio output
    let (_stream, stream_handle) =
        OutputStream::try_default().context("Failed to create audio output stream")?;

    let player = Arc::new(Player::new(&stream_handle, playlist, cli.volume)?);
    player.play_current()?;

    // Enable raw mode for keyboard input
    enable_raw_mode()?;

    // Event loop for handling keyboard input
    loop {
        if let Event::Key(key_event) = event::read()? {
            match key_event.code {
                KeyCode::Char('q') => break,
                KeyCode::Char('n') => player.next_track()?,
                KeyCode::Char('p') => player.previous_track()?,
                _ => {}
            }
        }

        // If current track finished, automatically play next
        if player.sink.empty() {
            player.next_track()?;
        }

        std::thread::sleep(Duration::from_millis(100));
    }

    // Cleanup
    disable_raw_mode()?;
    Ok(())
}

