mod application_error;
mod file;
mod parser;
mod pelada;

fn main() {
    let filepath: &str = "samples/sample2.dat";

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
}
