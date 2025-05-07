use std::io;

fn main() {
  let stdin = io::stdin();

  let mut a = 0;
  let mut g = 0;
  let mut d = 0;

  loop {
    let mut input = String::new();

    let n = match stdin.read_line(&mut input) {
      Ok(_) => input.trim()
        .parse()
        .unwrap_or(0),
      Err(_) => continue,
    };

    match n {
      1 => a += 1,
      2 => g += 1,
      3 => d += 1,
      4 => break,
      _ => continue,
    };
  }

  println!("MUITO OBRIGADO");
  println!("Alcool: {}", a);
  println!("Gasolina: {}", g);
  println!("Diesel: {}", d);
}