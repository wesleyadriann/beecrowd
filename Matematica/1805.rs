use std::io;

fn main() {
  let mut input = String::with_capacity(32);

  io::stdin().read_line(&mut input).unwrap();

  let mut values = input.split_whitespace();

  let start: u64 = values.next()
    .and_then(|x| x.parse().ok())
    .unwrap();

  let end: u64 = values.next()
    .and_then(|x| x.parse().ok())
    .unwrap();

  let n = end - start + 1;
  println!("{}", ((n * (start + end)) / 2));
}