use std::io;

fn main() {
  let mut input = String::new();

  io::stdin().read_line(&mut input).unwrap();

  let n: u32 = input.trim()
    .parse()
    .unwrap();

  for i in 1..=10000 {
    if i % n == 2 {
      println!("{}", i);
    }
  }
}