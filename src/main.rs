mod file;
mod parser;
mod pelada;

fn main() {
    let content: &str = &file::read_file("samples/sample.dat");

    let raw_list: Vec<String> = parser::run(content);
    
    for line in raw_list {
        println!("{}", line)
    }
}
