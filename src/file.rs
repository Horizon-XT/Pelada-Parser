use std::fs;
use std::io;

use crate::application_error;

fn check_valid_len(substrings: &Vec<&str>) -> Result<(), application_error::ApplicationError> {
    let invalid_input_error = application_error::ApplicationError::InvalidInput(
        "Please, certify that you are giving a valid and readable file.".to_string(),
    );

    if substrings.len() < 2 {
        return Err::<(), application_error::ApplicationError>(invalid_input_error);
    }

    Ok(())
}

fn split_extension(filename: &str) -> Result<&str, application_error::ApplicationError> {
    let invalid_input_error = application_error::ApplicationError::InvalidInput(
        "Please, certify that you are giving a valid and readable file.".to_string(),
    );
    let delimiter: &str = ".";

    let substrings: Vec<&str> = filename.split(delimiter).collect();

    check_valid_len(&substrings)?;

    match substrings.last() {
        Some(extension) => {
            if extension.is_empty() {
                return Err(invalid_input_error);
            } else {
                return Ok(extension);
            }
        }
        None => {
            return Err(invalid_input_error);
        }
    }
}

fn is_valid_extension(ext: &str) -> bool {
    match ext {
        "dat" | "txt" => true,
        _ => false,
    }
}

fn read_io(filename: &str) -> Result<String, application_error::ApplicationError> {
    let result: Result<String, io::Error> = fs::read_to_string(filename);

    match result {
        Ok(content) => {
            return Ok(content);
        }
        Err(error) => {
            let io_error =
                application_error::ApplicationError::IOError(format!("[{}]: {}", filename, error));
            return Err(io_error);
        }
    }
}

pub fn exist(filename: &str) -> bool {
    match fs::metadata(filename) {
        Ok(metadata) => {
            if metadata.is_file() {
                return true;
            }

            return false;
        }
        Err(_) => {
            return false;
        }
    }
}

pub fn read_file(filename: &str) -> Result<String, application_error::ApplicationError> {
    let unsupported_file_error = application_error::ApplicationError::UnsupportedFile;
    let ext: &str = split_extension(filename)?;

    if is_valid_extension(ext) {
        return read_io(filename);
    } else {
        return Err(unsupported_file_error);
    }
}
