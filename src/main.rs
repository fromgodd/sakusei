mod args;
mod errors;

use std::fs::File;
use std::path::PathBuf;
use std::{env, fs};
use args::parse_args;

fn try_main() -> Result<(), Box<dyn std::error::Error>> {
    let args = parse_args()?;
    for arg in &args {
        if arg.is_dir() {
            if arg.as_os_str().to_string_lossy().ends_with('/')
                || arg.as_os_str().to_string_lossy().ends_with('\\')
            {
                fs::create_dir_all(arg)?;
            } else {
                fs::create_dir_all(arg.parent().ok_or(errors::CustomError::InvalidParentDirectory)?)?;
                if !arg.exists() {
                    File::create(arg)?;
                }
            }
        } else if !arg.exists() {
            File::create(arg)?;
        }
    }
    Ok(())
}

fn main() {
    if let Err(e) = try_main() {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}
