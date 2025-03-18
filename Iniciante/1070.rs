use std::io;

fn main() {
  let mut input = String::new();

  io::stdin().read_line(&mut input).unwrap();

  let mut x: u32 = input.trim().parse().unwrap();

  let mut counter = 0;
  while counter < 6 {
    if x % 2 != 0 {
      println!("{}", x);
      counter += 1;
    }
    x += 1;
  }
}