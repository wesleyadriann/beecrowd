use std::io;

fn main() {
  let stdin = io::stdin();

  let mut input = String::new();

  let n = match stdin.read_line(&mut input) {
    Ok(_) => input.trim().parse().unwrap(),
    Err(_) => return,
  };

  for _ in 0..n {
    let mut input_x = String::new();

    let x: u32 = match stdin.read_line(&mut input_x) {
      Ok(_) => input_x.trim().parse().unwrap(),
      Err(_) => break,
    };

    println!("{} kg", (2u64.pow(x) - 1) / (12 * 1000));
  }
}