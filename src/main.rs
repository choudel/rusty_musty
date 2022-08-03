#![allow(unused)]
use rand::Rng;
use std::cmp::Ordering;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader, ErrorKind, Write};
fn main() {
  let mut st1=String::new();
  st1.push('A');
  st1.push_str("word");
  for word in st1.split_whitespace(){
    println!("{}",word);
  }
  let st2= st1.replace("A", "Another ");
  println!("{}",st2);
  let st3 = String::from("X v H f u IO u i");
  let mut v1: Vec<char> = st3.chars().collect();
  v1.sort();
  v1.dedup();
  for char in v1 {
      println!("{}",char);
  }
  let st4:&str="Random string";
  let mut st5:String=st4.to_string();
  println!("{}",st5);
  let byte_arr1 = st5.as_bytes();
  let st6 = &st5[1..4];
  println!("{}",st6);
  let st6 = String::from("Just some");
  let st7=String::from("words ");
  let st8 =st6+&st7;
  for char in st8.bytes(){
    println!("{}",char)
  }
}
