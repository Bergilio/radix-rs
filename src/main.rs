mod cli;

use clap::Parser;
use cli::Args;
use dicom::object::open_file;
use dicom::pixeldata::PixelDecoder;
use std::fs::create_dir_all;
use std::fs::read_dir;
use std::path::Path;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    create_dir_all(&args.out)?;
    println!("Welcome to Radix!");

    if args.path.is_dir() {
        let path_iter = read_dir(args.path)?.flatten().filter(|entry| {
            let file_name = entry.file_name().to_string_lossy().to_lowercase();
            file_name.ends_with(".dcm")
        });

        for path in path_iter {
            process_file(&path.path(), &args.out);
        }
    } else if args.path.is_file() {
        process_file(&args.path, &args.out);
    }

    Ok(())
}

fn process_file(path: &Path, out_dir: &String) {
    let file_name = match path.file_stem() {
        Some(name) => name,
        None => {
            println!("Error: Path has no file name!");
            return; // Exit the function early
        }
    };

    let file_name = file_name.to_string_lossy().to_string();

    let obj = match open_file(path) {
        Ok(obj) => obj,
        Err(_) => {
            println!("Failed to open {}", file_name);
            return;
        }
    };

    let image = match obj.decode_pixel_data() {
        Ok(image) => image,
        Err(_) => {
            println!("Failed to decode pixel data for {}", file_name);
            return;
        }
    };

    let dynamic_image = match image.to_dynamic_image(0) {
        Ok(dynamic_image) => dynamic_image,
        Err(_) => {
            println!("Failed to generate image from {}", file_name);
            return;
        }
    };

    dynamic_image
        .save(format!("{}/{}.png", out_dir, file_name))
        .expect("Failed to save image");
    println!("Processed {}", file_name);
}
