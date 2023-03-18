extern crate image;

use std::collections::HashMap;
use std::env;
use std::path::Path;
use image::{io::Reader as ImageReader, Rgb};

fn main() {
    // Get the command line arguments
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <image_file>", args[0]);
        return;
    }

    let file_path = &args[1];
    let path = Path::new(file_path);

    // Read the image file
    let image_result = ImageReader::open(path)
        .unwrap_or_else(|err| {
            eprintln!("Error opening file: {}", err);
            std::process::exit(1);
        })
        .with_guessed_format()
        .unwrap_or_else(|err| {
            eprintln!("Error guessing format: {}", err);
            std::process::exit(1);
        })
        .decode();

    let image = match image_result {
        Ok(image) => image,
        Err(err) => {
            eprintln!("Error decoding image: {}", err);
            return;
        }
    };

    // Convert the image to an RGB image
    let rgb_image = image.to_rgb8();
    let (width, height) = rgb_image.dimensions();

    // Create a HashMap to store the hex color frequencies
    let mut color_counts: HashMap<String, u32> = HashMap::new();

    // Iterate over each pixel
    for y in 0..height {
        for x in 0..width {
            let pixel = rgb_image.get_pixel(x, y);
            let hex_value = pixel_to_hex(pixel);

            // Increment the count for the current hex color
            let count = color_counts.entry(hex_value).or_insert(0);
            *count += 1;
        }
    }

    // Print the color frequencies
    for (color, count) in &color_counts {
        println!("Color {}: {} occurrences", color, count);
    }
}

fn pixel_to_hex(pixel: &Rgb<u8>) -> String {
    format!("#{:02X}{:02X}{:02X}", pixel[0], pixel[1], pixel[2])
}

fn hex_codes_distance(hex1: &str, hex2: &str) -> f64 {
    let (r1, g1, b1) = hex_to_rgb(hex1);
    let (r2, g2, b2) = hex_to_rgb(hex2);

    let r_diff = r1 as f64 - r2 as f64;
    let g_diff = g1 as f64 - g2 as f64;
    let b_diff = b1 as f64 - b2 as f64;

    ((r_diff * r_diff) + (g_diff * g_diff) + (b_diff * b_diff)).sqrt()
}

fn hex_to_rgb(hex: &str) -> (u8, u8, u8) {
    u32::from_str_radix(&hex[1..], 16)
        .map(|rgb| ((rgb >> 16) as u8, (rgb >> 8 & 0xFF) as u8, (rgb & 0xFF) as u8))
        .unwrap_or((0, 0, 0))
}
