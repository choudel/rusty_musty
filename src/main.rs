#![allow(unused)]
use rand::Rng;
use std::cmp::Ordering;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader, ErrorKind, Write};
fn main() {
    let num_1: f32 = 1.11111111111;
    println!("f32:{}", num_1 + 0.111111111111111);
    let random_num = rand::thread_rng().gen_range(5..25);
    println!("{}", random_num);
    let my_age = 47;
    let can_vote = if my_age >= 18 { true } else { false };
    println!("can vote: {}", can_vote);
    let voting_age = 18;
    match my_age.cmp(&voting_age) {
        Ordering::Less => println!("can't vote"),
        Ordering::Greater => println!("you can vote"),
        Ordering::Equal => println!(" you just made it to vote"),
    }
}
