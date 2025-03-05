use std::io;

fn main() {
  let mut input = String::new();

  io::stdin().read_line(&mut input).unwrap();

  let value = input.trim().parse::<f64>().unwrap();

  let tax = match value {
    0.0..=2000.0 => 0.0,
    2000.0..=3000.0 => (value - 2000.0) * 0.08,
    3000.0..=4500.0 => (value - 3000.0) * 0.18 + (1000.0 * 0.08),
    _ => (value - 4500.0) * 0.28 + (1500.0 * 0.18) + (1000.0 * 0.08),
  };

  if tax == 0.0 {
    println!("Isento");
  } else {
    println!("R$ {:.2}", tax);
  }
}