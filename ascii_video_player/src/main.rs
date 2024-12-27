use anyhow::{Context, Result};
use clap::Parser;
use crossterm::{
    cursor::{Hide, Show},
    event::{self, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::{
    io::{stdout, Write},
    path::PathBuf,
    sync::atomic::{AtomicBool, Ordering},
    sync::Arc,
    thread,
    time::{Duration, Instant},
};

mod frame_processor;
mod video_decoder;

use frame_processor::FrameProcessor;
use video_decoder::VideoDecoder;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Path to the video file
    #[arg(short, long)]
    input: PathBuf,

    /// Target FPS (default: original video FPS)
    #[arg(short, long)]
    fps: Option<u32>,
}

struct VideoPlayer {
    decoder: VideoDecoder,
    processor: FrameProcessor,
    target_fps: Option<u32>,
    running: Arc<AtomicBool>,
}

impl VideoPlayer {
    pub fn new(input_path: PathBuf, target_fps: Option<u32>) -> Result<Self> {
        let decoder = VideoDecoder::new(&input_path)?;
        let (width, height) = crossterm::terminal::size().unwrap_or((80, 24));
        let processor = FrameProcessor::new(width as u32, height as u32);
        let running = Arc::new(AtomicBool::new(true));

        Ok(Self {
            decoder,
            processor,
            target_fps,
            running,
        })
    }

    pub fn run(&mut self) -> Result<()> {
        // Setup terminal
        enable_raw_mode()?;
        execute!(stdout(), EnterAlternateScreen, Hide)?;

        // Setup interrupt handler
        let running = self.running.clone();
        ctrlc::set_handler(move || {
            running.store(false, Ordering::SeqCst);
        })?;

        let frame_duration =
            Duration::from_secs_f64(1.0 / self.target_fps.unwrap_or(self.decoder.get_fps()) as f64);

        while self.running.load(Ordering::SeqCst) {
            let frame_start = Instant::now();

            // Process events
            if event::poll(Duration::from_millis(1))? {
                if let Event::Key(key) = event::read()? {
                    match key.code {
                        KeyCode::Char('q') | KeyCode::Esc => break,
                        _ => {}
                    }
                }
            }

            // Decode and process frame
            if let Some(frame) = self.decoder.next_frame()? {
                let ascii_frame = self.processor.process_frame(&frame);
                print!("{}", ascii_frame);
                stdout().flush()?;
            } else {
                // End of video
                break;
            }

            // Frame timing
            let elapsed = frame_start.elapsed();
            if elapsed < frame_duration {
                thread::sleep(frame_duration - elapsed);
            }
        }

        // Cleanup
        execute!(stdout(), Show, LeaveAlternateScreen)?;
        disable_raw_mode()?;

        Ok(())
    }
}

fn main() -> Result<()> {
    let args = Args::parse();

    // Initialize FFmpeg
    ffmpeg_next::init().context("Failed to initialize FFmpeg")?;

    // Create and run video player
    let mut player = VideoPlayer::new(args.input, args.fps)?;
    player.run()?;

    Ok(())
}
