//use dicom::dictionary_std::tags;
use dicom::object::open_file;
use dicom::pixeldata::PixelDecoder;
use std::env::args;
use std::fs::read_dir;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Welcome to Radix!");
    let dir = args().nth(1).unwrap();
    let path_iter = read_dir(dir)?.flatten().filter(|entry| {
        let file_name = entry.file_name().to_string_lossy().to_lowercase();
        file_name.ends_with(".dcm")
    });

    for path in path_iter {
        let obj = match open_file(path.path()) {
            Ok(obj) => obj,
            Err(_) => {
                println!("Failed to open {}", path.file_name().to_string_lossy());
                continue;
            }
        };

        let image = match obj.decode_pixel_data() {
            Ok(image) => image,
            Err(_) => {
                println!(
                    "Failed to decode pixel data for {}",
                    path.file_name().to_string_lossy()
                );
                continue;
            }
        };
        let dynamic_image = match image.to_dynamic_image(0) {
            Ok(dynamic_image) => dynamic_image,
            Err(_) => {
                println!(
                    "Failed to generate image from {}",
                    path.file_name().to_string_lossy()
                );
                continue;
            }
        };

        let _ = dynamic_image.save(format!(
            "out/{}.png",
            path.file_name().to_string_lossy().trim_end_matches(".dcm")
        ));
        println!("Processed {}", path.file_name().to_string_lossy());
    }

    Ok(())
}
