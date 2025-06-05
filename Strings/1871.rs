use std::io::{
  self,
  BufRead,
  BufReader
};


fn main() {
  let mut reader = BufReader::new(io::stdin());

  let mut input = String::with_capacity(24);

  loop {
    input.clear();

    reader.read_line(&mut input).unwrap();

    if input.trim() == "0 0" {
      break;
    }

    let mut values = input.split_whitespace();

    let m: u32 = values.next()
      .and_then(|x| x.parse().ok())
      .unwrap();

    let n: u32 = values.next()
      .and_then(|x| x.parse().ok())
      .unwrap();

    let result = (m + n).to_string()
      .replace("0", "");

    println!("{}", result);

  }
}
