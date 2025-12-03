use std::io;

fn main() {
  let mut input = String::new();

  io::stdin().read_line(&mut input).unwrap();

  let value = input.trim().parse::<f64>().unwrap();

  let increment = match value {
    0.0..=400.00 => 0.15,
    400.00..=800.00 => 0.12,
    800.00..=1200.00 => 0.1,
    1200.00..=2000.00 => 0.07,
    _ => 0.04,
  };


  println!("Novo salario: {:.2}
Reajuste ganho: {:.2}
Em percentual: {:.0} %", value * (1.0 + increment), value * increment, increment * 100.0);
}