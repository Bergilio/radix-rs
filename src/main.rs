use dicom::dictionary_std::tags;
use dicom::object::open_file;
use dicom::pixeldata::PixelDecoder;
use std::fs::read_dir;
use std::env::args;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Welcome to Radix!");
    let dir = args().nth(1).unwrap();
    let path_iter = read_dir(dir)?.into_iter().flatten();

    for path in path_iter {

        let obj = open_file(path.path())?;

        let patient_name = obj.element(tags::PATIENT_NAME)?.to_str()?;
        let modality = obj.element(tags::MODALITY)?.to_str()?;

        println!("Patient name: {patient_name}");
        println!("Modality: {modality}");

        let rows = obj.element(tags::ROWS)?.uint16()?;
        let cols = obj.element(tags::COLUMNS)?.uint16()?;

        println!("Rows: {rows}");
        println!("Columns: {cols}");

        let image = obj.decode_pixel_data()?;
        let dynamic_image = image.to_dynamic_image(0)?;

        let _ = dynamic_image.save(format!("out/{:?}.png", path.file_name()));
    }


    Ok(())
}
