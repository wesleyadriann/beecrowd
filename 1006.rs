use std::io;

fn main() {
  let mut input_a = String::new();
  let mut input_b = String::new();
  let mut input_c = String::new();
  
  io::stdin().read_line(&mut input_a).unwrap();
  io::stdin().read_line(&mut input_b).unwrap();
  io::stdin().read_line(&mut input_c).unwrap();

  let a = input_a.trim().parse::<f64>().unwrap();
  let b = input_b.trim().parse::<f64>().unwrap();
  let c: f64 = input_c.trim().parse().unwrap();

  let result = ((a * 2.0) + (b * 3.0) + (c * 5.0)) / 10.0;

  println!("MEDIA = {:.1}", result);
}