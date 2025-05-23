use std::io::{
  self,
  BufRead,
  BufReader
};

fn main() {
  let mut reader = BufReader::new(io::stdin());
  let mut input = String::with_capacity(12);

  loop {
    input.clear();
    reader.read_line(&mut input).unwrap();
    let mut values = input.trim()
      .split_whitespace();

    let mut a: u32 = values.next()
      .and_then(|x| x.parse().ok())
      .unwrap();

    let mut b: u32 = values.next()
      .and_then(|x| x.parse().ok())
      .unwrap();

    if a == 0 && b == 0 {
      break;
    }

    let mut result: u32 = 0;
    let mut carry: u32 = 0;

    while a > 0 || b > 0 {
      if (a % 10) + (b % 10) + carry > 9 {
        carry = 1;
        result += 1;
      } else {
        carry = 0;
      }
      a /= 10;
      b /= 10;
    }

    match result {
      0 => println!("No carry operation."),
      1 => println!("1 carry operation."),
      _ => println!("{} carry operations.", result),
    };
  }
}