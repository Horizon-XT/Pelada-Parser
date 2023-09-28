use std::fs;
use std::io;

use crate::application_error;

fn split_extension(filename: &str) -> &str {
    let delimiter: &str = ".";
    let invalid_input_error = application_error::ApplicationError::InvalidInput(
        "Please, certify that you are giving a valid and readable file.".to_string(),
    );

    let substrings: Vec<&str> = filename.split(delimiter).collect();

    if substrings.len() < 2 {
        panic!("{}", invalid_input_error);
    }

    match substrings.last() {
        Some(extension) => {
            if extension.is_empty() {
                panic!("{}", invalid_input_error);
            } else {
                return extension;
            }
        }
        None => {
            panic!("{}", invalid_input_error);
        }
    }
}

fn is_valid_extension(ext: &str) -> bool {
    match ext {
        "dat" | "txt" => true,
        _ => false,
    }
}

fn read_io(filename: &str) -> String {
    let result: Result<String, io::Error> = fs::read_to_string(filename);

    match result {
        Ok(content) => {
            return content;
        }
        Err(error) => {
            panic!("Error reading {}!\nError: {}", filename, error);
        }
    }
}

pub fn read_file(filename: &str) -> String {
    let ext: &str = split_extension(filename);

    if is_valid_extension(ext) {
        return read_io(filename);
    } else {
        let unsupported_file_error = application_error::ApplicationError::UnsupportedFile;
        panic!("{}", unsupported_file_error);
    }
}
