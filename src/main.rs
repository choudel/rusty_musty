fn main() {
    let mut n = 0;
    while n <= 50 {
        if n % 5 == 0 {
            println!("n is {}--------------", n);
        } else {
            println!("n is {}", n);
        }

        n += 1;
    }
}
