fn main() {
    println!("hello world");
    test_one();
    test_one();
    add_numbers(3, 4);

    let number = {
        let x = 3;
        x + 1 //don't add a semicolon
    };
    println!("{}", number);
    println!("{}", add_numbers2(12, 13));
}

fn test_one() {
    println!("test has been called ...");
}
fn add_numbers(x: i32, y: i32) {
    println!("the sum is: {}", x + y);
}
fn add_numbers2(x: i32, y: i32) -> i32 {
    x + y
}
