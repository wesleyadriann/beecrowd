use std::io;

fn main() {
  let stdin = io::stdin();

  let mut input = String::new();

  let n: u32 = match stdin.read_line(&mut input) {
    Ok(_) => input.trim().parse().unwrap(),
    Err(_) => return,
  };

  for _ in 0..n {
    let mut input_line = String::new();
    stdin.read_line(&mut input_line).unwrap();

    let ab: Vec<&str> = input_line.split_whitespace()
      .collect();

    let (a, b) = (ab[0], ab[1]);

    if a.ends_with(&b) {
      println!("encaixa");
    } else {
      println!("nao encaixa");
    }
  }
}