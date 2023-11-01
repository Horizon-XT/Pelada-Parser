use std::env;

mod application_error;
mod argument;
mod file;
mod parser;
mod pelada;

fn main() {
    let args: Vec<String> = env::args().collect();

    argument::parse_args(args);

    // TODO: Pass as argument
    let filepath: &str = "samples/sample6.dat";

    if file::exist(filepath) {
        let content: &str;
        let read_file_result = &file::read_file(filepath);
        match &read_file_result {
            Ok(c) => {
                content = c;
            }
            Err(error) => {
                panic!("{}", error);
            }
        }

        let raw_list: Vec<String> = parser::run(content);

        let typed_pelada: pelada::PeladaType = pelada::from(raw_list);

        for element in &typed_pelada.goalkeepers {
            println!("{}", element)
        }
        for element in &typed_pelada.players {
            println!("{}", element)
        }
        for element in &typed_pelada.guests {
            println!("{}", element)
        }
        for element in &typed_pelada.kids {
            println!("{}", element)
        }

        match typed_pelada.to_json() {
            Ok(json) => {
                println!("{}", json);
            }
            Err(_err) => {
                println!("vishkkkkkk");
            }
        }
    } else {
        let file_not_found_error =
            application_error::ApplicationError::FileNotFound(filepath.to_string());
        panic!("{}", file_not_found_error)
    }
}
