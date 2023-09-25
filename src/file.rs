use std::fs;
use std::io;

fn split_extension(filename: &str) -> &str {
    let delimiter: &str = ".";

    let substrings: Vec<&str> = filename.split(delimiter).collect();

    match substrings.last() {
        Some(extension) => {
            return extension;
        }
        None => {
            return "TODO";
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
        panic!("Please, input a valid file! [.dat, .txt]");
    }
}
