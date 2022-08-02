#![allow(unused)]
use std::io;
use rand::Rng;
use std::io::{Write,BufReader,BufRead,ErrorKind};
use std::fs::File;
use std::cmp::Ordering;
fn main() {
  println!("what is your name ?");
  let mut name=String::new();
  let greeting="nice to meet you";
  io::stdin().read_line(&mut name).expect("didn't recieve input");
  println!("Hello {}! {}", name.trim_end(),greeting)
}
