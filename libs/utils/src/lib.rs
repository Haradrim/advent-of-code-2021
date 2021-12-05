use std::{fs};

pub fn read_file(path: &str) -> std::io::Result<String> {
    let input = fs::read_to_string(path)?;

    Ok(input)
}