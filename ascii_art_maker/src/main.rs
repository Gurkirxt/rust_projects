// use image::{imageops, DynamicImage, GenericImageView};
// use std::env;
// use std::fs;
//
// fn pixel_to_ascii(intensity: u8) -> char {
//     let ascii_chars = [' ', '.', ':', '-', '=', '+', '*', '#', '%', '@'];
//     let index = (intensity as f32 / 255.0 * (ascii_chars.len() - 1) as f32).round() as usize;
//     ascii_chars[index]
// }
//
// fn convert_to_ascii(image: &DynamicImage, binning: u32) -> String {
//     let (width, height) = image.dimensions();
//
//     // Ensure minimum dimensions
//     let scaled_width = (width / binning).max(1);
//     let scaled_height = ((height / binning) / 2).max(1);
//
//     let scaled_image =
//         image.resize_exact(scaled_width, scaled_height, imageops::FilterType::Nearest);
//
//     let mut ascii_art = String::new();
//     let (final_width, final_height) = scaled_image.dimensions();
//
//     // Always iterate from top to bottom
//     for y in 0..final_height {
//         for x in 0..final_width {
//             let pixel = scaled_image.get_pixel(x, y);
//             let intensity = ((pixel[0] as u32 + pixel[1] as u32 + pixel[2] as u32) / 3) as u8;
//             ascii_art.push(pixel_to_ascii(intensity));
//         }
//         ascii_art.push('\n');
//     }
//
//     ascii_art
// }
//
// fn main() -> Result<(), Box<dyn std::error::Error>> {
//     let args: Vec<String> = env::args().collect();
//
//     if args.len() != 4 {
//         println!(
//             "Usage: {} <input_image> <output_file> <binning_level>",
//             args[0]
//         );
//         return Ok(());
//     }
//
//     let input_path = &args[1];
//     let output_path = &args[2];
//     let binning = args[3].parse::<u32>()?;
//
//     if !(1..=100).contains(&binning) {
//         return Err("Binning level must be between 1 and 100".into());
//     }
//
//     let img = image::open(input_path)?;
//     let ascii_art = convert_to_ascii(&img, binning);
//
//     fs::write(output_path, &ascii_art)?;
//     println!("{}", ascii_art);
//
//     Ok(())
// }

use image::{imageops, DynamicImage, GenericImageView};
use std::env;
use std::fs;

fn pixel_to_ascii(intensity: u8) -> char {
    let ascii_chars = [' ', '.', ':', '-', '=', '+', '*', '#', '%', '@'];
    let index = (intensity as f32 / 255.0 * (ascii_chars.len() - 1) as f32).round() as usize;
    ascii_chars[index]
}

fn rgb_to_ansi_color(r: u8, g: u8, b: u8) -> String {
    format!("\x1b[38;2;{};{};{}m", r, g, b)
}

fn convert_to_ascii_with_color(image: &DynamicImage, binning: u32) -> String {
    let (width, height) = image.dimensions();

    // Ensure minimum dimensions
    let scaled_width = (width / binning).max(1);
    let scaled_height = ((height / binning) / 2).max(1);

    let scaled_image =
        image.resize_exact(scaled_width, scaled_height, imageops::FilterType::Nearest);

    let mut ascii_art = String::new();
    let (final_width, final_height) = scaled_image.dimensions();

    // Always iterate from top to bottom
    for y in 0..final_height {
        for x in 0..final_width {
            let pixel = scaled_image.get_pixel(x, y);
            let intensity = ((pixel[0] as u32 + pixel[1] as u32 + pixel[2] as u32) / 3) as u8;
            let ascii_char = pixel_to_ascii(intensity);

            // Apply color to the ASCII character
            let color = rgb_to_ansi_color(pixel[0], pixel[1], pixel[2]);
            ascii_art.push_str(&format!("{}{}", color, ascii_char));
        }
        ascii_art.push('\n');
    }

    // Reset terminal color
    ascii_art.push_str("\x1b[0m");
    ascii_art
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();

    if args.len() != 4 {
        println!(
            "Usage: {} <input_image> <output_file> <binning_level>",
            args[0]
        );
        return Ok(());
    }

    let input_path = &args[1];
    let output_path = &args[2];
    let binning = args[3].parse::<u32>()?;

    if !(1..=100).contains(&binning) {
        return Err("Binning level must be between 1 and 100".into());
    }

    let img = image::open(input_path)?;
    let ascii_art = convert_to_ascii_with_color(&img, binning);

    // Write colored ASCII art with ANSI codes to the file
    fs::write(output_path, &ascii_art)?;

    // Print colored ASCII art to the console
    println!("{}", ascii_art);

    Ok(())
}
