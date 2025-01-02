use anyhow::{Context, Result};
use clap::Parser;
use image::ImageFormat;
use std::path::Path;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[arg(short, long)]
    input: String,

    #[arg(short, long)]
    output: String,

    #[arg(short = 'w', long)]
    width: Option<u32>,

    #[arg(short = 't', long = "height")]
    height: Option<u32>,
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    let img =
        image::open(&cli.input).with_context(|| format!("Failed to open image: {}", cli.input))?;

    let (width, height) = match (cli.width, cli.height) {
        (Some(w), Some(h)) => (w, h),
        (Some(w), None) => {
            let ratio = w as f32 / img.width() as f32;
            (w, (img.height() as f32 * ratio) as u32)
        }
        (None, Some(h)) => {
            let ratio = h as f32 / img.height() as f32;
            ((img.width() as f32 * ratio) as u32, h)
        }
        (None, None) => (img.width(), img.height()),
    };

    let resized = img.resize(width, height, image::imageops::FilterType::Lanczos3);

    let format = match Path::new(&cli.output).extension().and_then(|s| s.to_str()) {
        Some("jpg") | Some("jpeg") => ImageFormat::Jpeg,
        Some("png") => ImageFormat::Png,
        Some("gif") => ImageFormat::Gif,
        Some("webp") => ImageFormat::WebP,
        _ => return Err(anyhow::anyhow!("Unsupported output format")),
    };

    resized
        .save_with_format(&cli.output, format)
        .with_context(|| format!("Failed to save image: {}", cli.output))?;

    Ok(())
}
