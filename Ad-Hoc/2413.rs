use std::io;

fn main() {
  let mut input = String::new();

  io::stdin().read_line(&mut input).unwrap();

  let t: u32 = input.trim()
    .parse()
    .unwrap();

  println!("{}", t * 4);
}