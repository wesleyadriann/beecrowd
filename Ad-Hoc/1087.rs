use std::io::{
  self,
  BufRead,
  BufReader
};

fn main() {
  let mut reader = BufReader::new(io::stdin());
  let mut input = String::with_capacity(8);

  loop {
    input.clear();
    reader.read_line(&mut input).unwrap();

    if input.trim() == "0 0 0 0" {
      break
    }

    let mut values = input.split_whitespace();

    let x1: u32 = values.next()
      .and_then(|x| x.parse().ok())
      .unwrap();

    let y1: u32 = values.next()
      .and_then(|x| x.parse().ok())
      .unwrap();

    let x2: u32 = values.next()
      .and_then(|x| x.parse().ok())
      .unwrap();

    let y2: u32 = values.next()
      .and_then(|x| x.parse().ok())
      .unwrap();

    if x1 == x2 && y1 == y2 {
      println!("0");
    } else if x1 == x2 || y1 == y2 || (x1 as i32 - x2 as i32).abs() == (y1 as i32 - y2 as i32).abs()  {
      println!("1");
    } else {
      println!("2");
    }
  }
}