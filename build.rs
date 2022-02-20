use std::env;
use std::fs;
use std::path::{Path, PathBuf};

fn main() -> std::io::Result<()> {
    let file_path = Path::new("src/filepaths.json");

    let out = &env::var("PROFILE").unwrap();
    let json_output_path = PathBuf::from(format!("target/{}/filepaths.json", out));

    fs::copy(file_path, json_output_path)?;
    Ok(())
}