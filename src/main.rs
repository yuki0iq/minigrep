use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    let query = &args[1];
    let file_path = &args[2];

    println!("find {} in file {}", query, file_path);

    let contents = fs::read_to_string(file_path)
        .expect("file should be available to read");

    println!("with text {}", contents);
}
