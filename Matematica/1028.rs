use std::io;

fn main() {
  let stdin = io::stdin();

  let mut input = String::new();

  let n = match stdin.read_line(&mut input) {
    Ok(_) => input.trim().parse().unwrap(),
    Err(_) => return
  };

  for _ in 0..n {
    let mut input_f = String::new();

    let values: Vec<u32> = match stdin.read_line(&mut input_f) {
      Ok(_) => input_f.split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect(),
      Err(_) => break,
    };

    let (f1, f2) = (values[0], values[1]);

    println!("{}", mdc(f1, f2));
  }
}

fn mdc(n1: u32, n2: u32) -> u32 {
  let mut a = n1;
  let mut b = n2;
  while b != 0 {
    let r = a % b;
    a = b;
    b = r;
  }
  return a;
}