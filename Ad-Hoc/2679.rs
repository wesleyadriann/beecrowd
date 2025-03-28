use std::io;

fn main() {
  let mut input = String::new();

  io::stdin().read_line(&mut input).unwrap();

  let x: u32 = input.trim()
    .parse()
    .unwrap();

  let mut sum_next: u32 = 1;
  if x % 2 == 0 {
    sum_next = 2;
  }

  println!("{}", x + sum_next);
}