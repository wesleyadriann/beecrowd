use std::io;

fn main() {
  let mut input = String::new();

  io::stdin().read_line(&mut input).unwrap();

  let values: Vec<&str> = input.split_whitespace().collect();

  let a = values[0].parse::<f64>().unwrap();
  let b = values[1].parse::<f64>().unwrap();
  let c = values[2].parse::<f64>().unwrap();

  if a < b + c &&
    b < a + c &&
    c < a + b {
    println!("Perimetro = {:.1}", a + b + c);
  } else {
    println!("Area = {:.1}", (a + b) * c / 2.0);
  }
}