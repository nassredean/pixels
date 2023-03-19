extern crate image;
extern crate ndarray;
extern crate rand;

use std::collections::HashMap;
use std::env;
use std::path::Path;
use image::{io::Reader as ImageReader, Rgb};
use ndarray::{Array, Array1, array};
use rand::seq::SliceRandom;

fn main() {
    // Get the command line arguments
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        eprintln!("Usage: {} <image_file> <num_buckets>", args[0]);
        return;
    }

    let file_path = &args[1];
    let path = Path::new(file_path);

    let num_buckets: usize = args[2].parse().unwrap_or_else(|_| {
        eprintln!("Invalid number of buckets: {}", args[2]);
        std::process::exit(1);
    });

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

    // Convert the color_counts HashMap into a Vec of Array1<f64>
    let colors: Vec<Array1<f64>> = color_counts
        .keys()
        .map(|hex| {
            let (r, g, b) = hex_to_rgb(hex);
            array![r as f64, g as f64, b as f64]
        })
        .collect();

    let clusters = k_means(&colors, num_buckets);

    // Print the clustered colors
    for (i, cluster) in clusters.iter().enumerate() {
        println!("Cluster {}:", i + 1);
        for color in cluster {
            let hex_color = rgb_to_hex(color[0] as u8, color[1] as u8, color[2] as u8);

            print_colored_hex(&hex_color);
        }
        println!(); // Add a newline after each cluster
    }
}

fn pixel_to_hex(pixel: &Rgb<u8>) -> String {
    format!("#{:02X}{:02X}{:02X}", pixel[0], pixel[1], pixel[2])
}

fn hex_to_rgb(hex: &str) -> (u8, u8, u8) {
    u32::from_str_radix(&hex[1..], 16)
        .map(|rgb| ((rgb >> 16) as u8, (rgb >> 8 & 0xFF) as u8, (rgb & 0xFF) as u8))
        .unwrap_or((0, 0, 0))
}

fn print_colored_hex(hex_color: &str) {
    let (r, g, b) = hex_to_rgb(hex_color);
    print!("\x1b[38;2;{};{};{}m{}\x1b[0m ", r, g, b, hex_color);
}

fn rgb_to_hex(r: u8, g: u8, b: u8) -> String {
    format!("#{:02X}{:02X}{:02X}", r, g, b)
}

fn k_means(colors: &[Array1<f64>], num_buckets: usize) -> Vec<Vec<Array1<f64>>> {
    // Randomly select initial centroids
    let mut rng = rand::thread_rng();
    let mut centroids: Vec<Array1<f64>> = colors.choose_multiple(&mut rng, num_buckets).cloned().collect();

    let mut cluster_assignments = vec![0; colors.len()];
    let mut prev_assignments = vec![usize::MAX; colors.len()];

    while cluster_assignments != prev_assignments {
        // Assign colors to the nearest centroid
        for (i, color) in colors.iter().enumerate() {
            let mut min_distance = f64::MAX;
            let mut min_index = 0;

            for (j, centroid) in centroids.iter().enumerate() {
                let distance = (centroid - color).dot(&(centroid - color));
                if distance < min_distance {
                    min_distance = distance;
                    min_index = j;
                }
            }

            prev_assignments[i] = cluster_assignments[i];
            cluster_assignments[i] = min_index;
        }

        // Update centroids
        centroids = calculate_centroids(&colors, &cluster_assignments, num_buckets);
    }

    // Create the final color clusters
    let mut clusters: Vec<Vec<Array1<f64>>> = vec![Vec::new(); num_buckets];
    for (i, color) in colors.iter().enumerate() {
        clusters[cluster_assignments[i]].push(color.clone());
    }

    clusters
}

fn calculate_centroids(
    colors: &[Array1<f64>],
    cluster_assignments: &[usize],
    num_buckets: usize,
) -> Vec<Array1<f64>> {
    let mut new_centroids: Vec<Array1<f64>> = vec![Array::zeros(3); num_buckets];
    let mut counts = vec![0; num_buckets];

    for (i, color) in colors.iter().enumerate() {
        new_centroids[cluster_assignments[i]] += color;
        counts[cluster_assignments[i]] += 1;
    }

    for (i, centroid) in new_centroids.iter_mut().enumerate() {
        *centroid /= counts[i] as f64;
    }

    new_centroids
}