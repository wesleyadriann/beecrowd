use std::io::{
  self,
  BufRead,
  BufReader
};

fn main() {
  let mut reader = BufReader::new(io::stdin());
  let mut input = String::with_capacity(32);

  loop {
    input.clear();
    reader.read_line(&mut input).unwrap();

    if input.trim() == "0 0" {
      break;
    }

    let mut values = input.split_whitespace();

    let x: u32 = values.next()
      .and_then(|x| x.parse().ok())
      .unwrap();

    let m: u32 = values.next()
      .and_then(|x| x.parse().ok())
      .unwrap();

    println!("{}", x * m);

  }
}