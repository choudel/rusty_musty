#![allow(unused)]
use rand::Rng;
use std::cmp::Ordering;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader, ErrorKind, Write};
fn main() {
 enum Days {
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
    Sunday
 }
 impl Days {
  fn is_weekend(&self)-> bool{
    match self {
      Days::Saturday | Days::Sunday => true,
      _=>false,
    }

  }
}
let today:Days = Days::Monday;
  match today {
    Days::Friday=> println!("Not very nice day"),
    Days::Monday=> println!("thanks for it"),
    Days::Saturday=>println!("wierd day"),
    Days::Sunday=>println!("party night "),
    Days::Thursday=>println!("best days"),
    Days::Tuesday=>println!("midweek"),
    Days::Wednesday=>println!("weekday")
  }
  println!("is today the weekend {}",today.is_weekend());
 
}
