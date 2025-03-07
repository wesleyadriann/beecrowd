use std::io;

fn main() {
  let mut input_km = String::new();
  let mut input_l = String::new();

  io::stdin().read_line(&mut input_km).unwrap();
  io::stdin().read_line(&mut input_l).unwrap();

  let km = input_km.trim().parse::<f64>().unwrap();
  let l = input_l.trim().parse::<f64>().unwrap();

  println!("{:.3} km/l", km/l);
}