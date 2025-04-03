use std::io;

fn main() {
  let stdin = io::stdin();

  let mut input = String::new();

  let n: u16 = match stdin.read_line(&mut input) {
    Ok(_) => input.trim().parse().unwrap(),
    Err(_) => return,
  };

  for _ in 0..n {
    let mut input_c = String::new();

    let mut c: f64 = match stdin.read_line(&mut input_c) {
      Ok(_) => input_c.trim().parse().unwrap(),
      Err(_) => break,
    };

    let mut result: u16 = 0;
    while c > 1.0 {
      c = c / 2.0;
      result += 1;
    }

    println!("{} dias", result);
  }
}