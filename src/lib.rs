use std::{
    error::Error,
    ffi::OsStr,
    fs::File,
    io::{self, BufRead},
    path::Path,
};

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

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

pub fn run(config: InputConfig) -> Result<(), Box<dyn Error>> {
    if let Ok(lines) = read_lines(config.file_path) {
        for line in lines {
            if let Ok(ip) = line {
                println!("{}\n", ip);
            }
        }
    }

    Ok(())
}
