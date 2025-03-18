use std::io;

fn main() {
  let mut input = String::new();

  io::stdin().read_line(&mut input).unwrap();

  let x: u16 = input.trim().parse().unwrap();

  for i in 0..=x {
    if i % 2 != 0  {
      println!("{}", i);
    }
  }
}