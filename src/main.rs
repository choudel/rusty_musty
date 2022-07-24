fn main() {
    let mut my_string = String::from("helllo man banner");

    println!("Length:{}", my_string.len());
    println!("String is empty ? {}", my_string.is_empty());

    for token in my_string.split_whitespace() {
        println!("//========>{}<========//", token);
    }
    println!(
        "Does the string contain 'man'? {} ",
        my_string.contains("man")
    );
    my_string.push_str(" Welcome to strings!");
    println!("Same thing: {}", my_string);
}
