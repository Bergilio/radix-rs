use dicom::dictionary_std::tags;
use dicom::object::open_file;
//use dicom::dump::dump_file;
use dicom::pixeldata::PixelDecoder;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let obj = open_file("dicom-dataset/693_J2KI.dcm")?;

    let patient_name = obj.element(tags::PATIENT_NAME)?.to_str()?;
    let modality = obj.element(tags::MODALITY)?.to_str()?;

    println!("Welcome to Radix!");
    println!("Patient name: {patient_name}");
    println!("Modality: {modality}");

    // let _ = dump_file(&obj);

    let rows = obj.element(tags::ROWS)?.uint16()?;
    let cols = obj.element(tags::COLUMNS)?.uint16()?;

    println!("Rows: {rows}");
    println!("Columns: {cols}");

    let image = obj.decode_pixel_data()?;
    let dynamic_image = image.to_dynamic_image(0)?;

    let _ = dynamic_image.save("out/out.png");


    Ok(())
}
