use std::env;
use std::fs;
use std::path::{Path, PathBuf};

fn main() -> std::io::Result<()> {
    let out = &env::var("PROFILE").unwrap();
    let mut out_dir = PathBuf::from(format!("target/{}", out));
    out_dir.push("filepaths");
    out_dir.set_extension("json");

    let file_path = Path::new("src/filepaths.json");
    fs::copy(file_path.to_str().unwrap(), out_dir.to_str().unwrap())?;
    Ok(())
}