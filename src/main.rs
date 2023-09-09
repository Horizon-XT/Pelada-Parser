mod file;
mod parser;

fn main() {
    let content: &str = &file::read_file("samples/sample.dat");

    parser::run(content);
}
