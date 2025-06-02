use std::io::{
  self,
  BufRead,
  BufReader
};

fn main() {
  let mut reader = BufReader::new(io::stdin());

  let mut input = String::with_capacity(14);

  loop {
    input.clear();

    reader.read_line(&mut input).unwrap();

    if &input[0..1] == "0" &&
      &input[2..3] == "0" &&
      &input[4..5] == "0" &&
      &input[6..7] == "0" {
      break;
    }

    let mut values = input.split_whitespace();


    let h1: u32 = values.next()
      .and_then(|x| x.parse().ok())
      .unwrap();

    let m1: u32 = values.next()
      .and_then(|x| x.parse().ok())
      .unwrap();

    let h2: u32 = values.next()
      .and_then(|x| x.parse().ok())
      .unwrap();

    let m2: u32 = values.next()
      .and_then(|x| x.parse().ok())
      .unwrap();

    let minutes1 = h1 * 60 + m1;
    let minutes2 = h2 * 60 + m2;

    let result = if minutes1 > minutes2 {
      (minutes2 + 1440) - minutes1
    } else {
      minutes2 - minutes1
    };

    println!("{}", result);
  }
}