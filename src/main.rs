use dicom_dictionary_std::tags;
use dicom_object::open_file;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let obj = open_file("dicom-dataset/693_J2KI.dcm")?;

    let patient_name = obj.element(tags::PATIENT_NAME)?.to_str()?;
    let modality = obj.element_by_name("Modality")?.to_str()?;

    println!("Patient name: {patient_name}");
    println!("Modality: {modality}");

    Ok(())
}
