struct Color{
    red:u8,
    blue:u8,
    green:u8
}

fn main() {
    let blue = Color{red:2,green:41,blue:69};
    print_color(&blue);
    print_color(&blue);
}
fn print_color(c:&Color) {
    println!("Color R:{} G:{} B:{}",c.red, c.green,c.blue)
}