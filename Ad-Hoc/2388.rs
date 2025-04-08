use std::io;

fn main() {
  let stdin = io::stdin();

  let mut input = String::new();

  let n: u16 = match stdin.read_line(&mut input) {
    Ok(_) => input.trim().parse().unwrap(),
    Err(_) => return,
  };

  let mut result = 0;
  for _ in 0..n {
    let mut input_values = String::new();

    let values: Vec<u32> = match stdin.read_line(&mut input_values) {
      Ok(_) => input_values.split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect(),
      Err(_) => break,
    };

    result += values[0] * values[1];
  }

  println!("{}", result);
}