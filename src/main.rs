struct Color {
    red: u8,
    green: u8,
    blue: u8,
}
fn main() {
    let bg = Color {
        blue: 21,
        green: 85,
        red: 63,
    };
    println!("{},{},{}", bg.blue, bg.green, bg.red);
}
