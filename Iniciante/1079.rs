use std::io;

fn main() {
  let stdin = io::stdin();

  let mut input = String::new();

  let n: u32 = match stdin.read_line(&mut input) {
    Ok(_) => input.trim().parse().unwrap(),
    Err(_) => return
  };

  for _ in 0..n {
    let mut input_values = String::new();

    let values: Vec<f64> = match stdin.read_line(&mut input_values) {
      Ok(_) => input_values.split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect(),
      Err(_) => break,
    };

    let (a, b, c) = (values[0], values[1], values[2]);

    println!("{:.1}", (a * 2.0 + b * 3.0 + c * 5.0) / 10.0);
  }
}