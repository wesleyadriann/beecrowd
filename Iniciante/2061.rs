use std::io::{
  self,
  BufRead,
  BufReader
};

fn main() {
  let mut reader = BufReader::new(io::stdin());

  let mut input = String::with_capacity(8);

  reader.read_line(&mut input).unwrap();

  let mut values = input.split_whitespace();

  let mut n: u32 = values.next()
    .and_then(|x| x.parse().ok())
    .unwrap();

  let m: u32 = values.next()
    .and_then(|x| x.parse().ok())
    .unwrap();

  for _ in 0..m {
    input.clear();

    reader.read_line(&mut input).unwrap();

    if input.trim() == "fechou" {
      n += 1
    } else {
      n -= 1
    }
  }

  println!("{}", n);
}