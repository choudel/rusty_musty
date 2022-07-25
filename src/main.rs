use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};
fn main() {
    let mut rng = rand::thread_rng();
    let n1: u8 = rng.gen();
    let n2: u16 = rng.gen();
    let n3: u8 = rng.gen_range(0..10);
    println!("Random u8 : {}", n1);
    println!("Random u16: {}", n2);
    println!("Random between 0 and 10 : {}", n3);

    let rand_string: String = thread_rng()
        .sample_iter(&Alphanumeric)
        .take(20)
        .map(char::from)
        .collect();
    println!("{}", rand_string);
    let mut vec = vec![5, 1, 10, 2, 15];
    vec.sort();
    assert_eq!(vec, vec![1, 2, 5, 10, 15]);

    let mut vec1 = vec![1.1, 1.15, 5.5, 1.123, 2.0];
    vec1.sort_by(|a, b| a.partial_cmp(b).unwrap());
    assert_eq!(vec1, vec![1.1, 1.123, 1.15, 2.0, 5.5]);
}
