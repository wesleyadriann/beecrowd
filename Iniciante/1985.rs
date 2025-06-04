use std::io::{
  self,
  BufRead,
  BufReader
};

fn main() {
  let mut reader = BufReader::new(io::stdin());
  let mut input = String::with_capacity(10);

  reader.read_line(&mut input).unwrap();

  let n = input.trim()
    .parse()
    .unwrap_or(0);

  let mut result = 0.0;
  for _ in 0..n {
    input.clear();
    reader.read_line(&mut input).unwrap();

    let mut values = input.split_whitespace();

    let p = values.next()
      .unwrap();

    let q: f64 = values.next()
      .and_then(|x| x.parse().ok())
      .unwrap();

    let value = match p {
      "1001" => 1.50,
      "1002" => 2.50,
      "1003" => 3.50,
      "1004" => 4.50,
      "1005" => 5.50,
      _ => 0.0,
    };

    result += value * q;
  }

  println!("{:.2}", result);
 
}