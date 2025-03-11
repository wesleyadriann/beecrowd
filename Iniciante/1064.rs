use std::io;

fn main() {
  let mut input = String::new();

  io::stdin().read_line(&mut input).unwrap();
  io::stdin().read_line(&mut input).unwrap();
  io::stdin().read_line(&mut input).unwrap();
  io::stdin().read_line(&mut input).unwrap();
  io::stdin().read_line(&mut input).unwrap();
  io::stdin().read_line(&mut input).unwrap();

  let values: Vec<f64> = input.trim()
    .split("\n")
    .map(|x| x.trim().parse().unwrap())
    .collect();

  let mut pn: f64 = 0.0;
  let mut med: f64 = 0.0;

  for value in values {
    if value > 0.0 {
      pn += 1.0;
      med = med + value;
    }
  }

  println!("{} valores positivos
{:.1}", pn, med / pn);
}