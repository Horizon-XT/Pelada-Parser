use std::env;

mod application_error;
mod argument;
mod file;
mod parser;
mod pelada;

fn main() {
    let args: Vec<String> = env::args().collect();

    let (operation, input) = argument::parse_args(args);

    println!("\n[PARSER] Operation: {} | Input: {}\n", operation, input);

    match operation {
        argument::OperationMode::CLI => {
            let content: &str;
            let read_file_result = &file::read_file(&input);
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

            //for element in &typed_pelada.goalkeepers {
            //    println!("{}", element)
            //}
            //for element in &typed_pelada.players {
            //    println!("{}", element)
            //}
            //for element in &typed_pelada.guests {
            //    println!("{}", element)
            //}
            //for element in &typed_pelada.kids {
            //    println!("{}", element)
            //}

            match typed_pelada.to_json() {
                Ok(json) => {
                    //println!("{}", json);
                    file::write_file(json);
                }
                Err(_err) => {
                    println!("vishkkkkkk");
                }
            }
        }
        argument::OperationMode::API => {
            println!("Starting API...\n");
        }
        argument::OperationMode::Service => {
            println!("Starting Service...\n");
        }
    }
}
