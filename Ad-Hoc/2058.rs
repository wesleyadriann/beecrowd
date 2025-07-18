use std::io;

fn main() {
  let mut input = String::with_capacity(12);

  io::stdin().read_line(&mut input).unwrap();

  let n: u32 = input.trim()
    .parse()
    .unwrap();

  println!("{}", n - 2);
}