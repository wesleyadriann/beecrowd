use std::io::{
  self,
  BufRead,
  BufReader
};

fn main() {
  let mut reader = BufReader::new(io::stdin());
  let mut input = String::with_capacity(32);

  reader.read_line(&mut input).unwrap();

  let t: u16 = input.trim()
    .parse()
    .unwrap_or(0);

  for _ in 0..t {
    input.clear();
    reader.read_line(&mut input).unwrap();

    let mut values = input.split_whitespace();

    let r1: u32 = values.next()
      .and_then(|x| x.parse().ok())
      .unwrap();

    let r2: u32 = values.next()
      .and_then(|x| x.parse().ok())
      .unwrap();

    println!("{}", r1 + r2);
  }
}