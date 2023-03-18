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