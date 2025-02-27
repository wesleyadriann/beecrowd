use std::io;

fn main() {
  let mut input = String::new();

  io::stdin().read_line(&mut input).unwrap();

  let vec: Vec<&str> = input.split_whitespace().collect();

  let a = vec[0].parse::<f64>().unwrap();
  let b = vec[1].parse::<f64>().unwrap();
  let c = vec[2].parse::<f64>().unwrap();

  let delta = (b * b) - 4.0 * a * c;

  if  a < 1.0 || delta < 0.0 {
    println!("Impossivel calcular");
  } else {
    let r1 = (-1.0 * b + delta.sqrt()) / (2.0 * a);
    let r2 = (-1.0 * b - delta.sqrt()) / (2.0 * a);

    println!("R1 = {:.5}
R2 = {:.5}", r1, r2);
  }
}