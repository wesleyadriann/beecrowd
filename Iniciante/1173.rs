use std::io;

fn main() {
  let mut input = String::new();

  io::stdin().read_line(&mut input).unwrap();

  let v: u16 = input.trim()
    .parse()
    .unwrap();

  let mut prev = v;
  for i in 0..10 {
    println!("N[{}] = {}", i, prev);
    prev *= 2;
  }
}