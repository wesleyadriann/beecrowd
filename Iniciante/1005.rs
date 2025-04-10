use std::io;

fn main() {
  let mut input_a = String::new();
  let mut input_b = String::new();

  io::stdin().read_line(&mut input_a).unwrap();
  io::stdin().read_line(&mut input_b).unwrap();

  let a = input_a.trim().parse::<f64>().unwrap();
  let b = input_b.trim().parse::<f64>().unwrap();

  let median = ((a * 3.5) + (b * 7.5))/ 11.0;

  println!("MEDIA = {:.5}", median);
}