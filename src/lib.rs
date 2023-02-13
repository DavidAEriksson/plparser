use std::{error::Error, ffi::OsStr, fs, path::Path};

pub struct InputConfig {
    pub file_path: String,
}

impl InputConfig {
    pub fn build(args: &[String]) -> Result<InputConfig, &'static str> {
        if args.len() <= 1 {
            return Err("Missing required file.");
        }

        let file_path = args[1].clone();

        if check_extension(&file_path) != Some("csv") {
            return Err("Incorrect file extension.");
        }

        Ok(InputConfig { file_path })
    }
}

fn check_extension(file_name: &str) -> Option<&str> {
    Path::new(file_name).extension().and_then(OsStr::to_str)
}

pub fn run(config: InputConfig) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;

    println!("With text:\n{contents}");

    Ok(())
}
