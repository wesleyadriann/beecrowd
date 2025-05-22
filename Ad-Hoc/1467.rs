use std::io::{
  self,
  BufRead,
  BufReader
};

fn main() {
  let mut reader = BufReader::new(io::stdin());

  let mut input = String::new();

  while reader.read_line(&mut input).unwrap() > 0 {
    let mut values = input.split_whitespace();

    let a: u32 = values.next()
      .and_then(|x| x.parse().ok())
      .unwrap();

    let b: u32 = values.next()
      .and_then(|x| x.parse().ok())
      .unwrap();

    let c: u32 = values.next()
      .and_then(|x| x.parse().ok())
      .unwrap();

    if a == b && b == c {
      println!("*");
    } else if a != b && b == c  {
      println!("A");
    } else if b != a && a == c {
      println!("B");
    } else {
      println!("C");
    }
    input.clear();
  }
}