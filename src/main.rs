mod file;

fn main() {
    let content: &str = &file::read_file("samples/sample.dat");

    println!("File content: {}", content);
}
