use std::fs::File;
use std::io::prelude::*;
fn main() {
    let mut file= File::open("info.txt").expect("can't read file!");
    let mut content = String::new();
    file.read_to_string(&mut content).expect("Oops! Can not read the file...");
    println!("File Contents : \n\n {}",content);
}
