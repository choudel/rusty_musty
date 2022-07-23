use std::io;
fn main() {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("there is a problem");
    let int_input: i64 = input.trim().parse().unwrap();
    println!("the raw input is : {}", input);
    println!("the converted input is : {}", int_input + 2);
}
