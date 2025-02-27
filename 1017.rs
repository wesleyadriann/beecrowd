use std::io;

fn main() {

  let mut input_t = String::new();
  let mut input_v = String::new();

  io::stdin().read_line(&mut input_t).unwrap();
  io::stdin().read_line(&mut input_v).unwrap();

  let t = input_t.trim().parse::<f64>().unwrap();
  let v = input_v.trim().parse::<f64>().unwrap();

  println!("{:.3}", t * v / 12.0);
}