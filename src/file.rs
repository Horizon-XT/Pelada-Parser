//use std::fs::File;
//use std::io::{self, Read};

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

pub fn read_file(filename: &str) {
    println!("read file: {}", filename);

    let ext: &str = split_extension(filename);

    if is_valid_extension(ext) {
        println!("the extension {} is valid? true", ext);
    } else {
        println!("the extension {} is valid? false", ext);
    }
} 
