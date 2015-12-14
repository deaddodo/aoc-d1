use std::io::prelude::*;
use std::fs::File;

fn main() {
  let f = match File::open("input.txt") {
    Err(e) => panic!("{}", e),
    Ok(f) => f
  };
  let mut floor:i32 = 0;

  for ch in f.bytes().filter_map(|result| result.ok()) {
    floor += match ch {
      40 => 1,
      41 => -1,
      _ => 0
    }
  }

  println!("Floor number: {}", floor);
}
