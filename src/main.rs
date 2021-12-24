use std::{env, fs};

fn main() {
    let args = env::args().collect::<Vec<String>>();
    let query = &args[1];
    let filename = &args[2];

    println!("Query: {}", query);
    println!("File: {}", filename);

    let content =
        fs::read_to_string(filename).expect("Something went wrong with reading the file.");

    println!("Content: {}", content);
}
