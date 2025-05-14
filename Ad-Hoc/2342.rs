use std::io;

fn main() {
  let stdin = io::stdin();

  let mut input = String::new();

  stdin.read_line(&mut input).unwrap();

  let n: u32 = input.trim()
    .parse()
    .unwrap();

  input.clear();
  stdin.read_line(&mut input).unwrap();

  let values: Vec<&str> = input.split_whitespace()
    .collect();

  let p: u32 = values[0].parse().unwrap();
  let q: u32 = values[2].parse().unwrap();
  let c = values[1];

  let result = match c {
    "+" => p + q,
    _ => p * q,
  };

  if result > n {
    println!("OVERFLOW");
  } else {
    println!("OK");
  }
}