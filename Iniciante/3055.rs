use std::io;

fn main() {
  let stdin = io::stdin();
  let mut input = String::with_capacity(6);

  stdin.read_line(&mut input).unwrap();

  let a: u16 = input.trim()
    .parse()
    .unwrap();

  input.clear();
  stdin.read_line(&mut input).unwrap();

  let m: u16 = input.trim()
    .parse()
    .unwrap();

  println!("{}", 2 * m - a);
}