use std::fs;

fn main() {
    fs::remove_file("data.pdf").expect("could not remove file");
    println!("file is removed");
}