use clap::Parser;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(version, about, long_about=None)]
pub struct Args {
    /// The input file or directory containing the dicom fies
    pub path: PathBuf,

    /// The output directory where extracted PNGs will be saved
    #[arg(short, long, default_value_t = String::from("out"))]
    pub out: String,
}
