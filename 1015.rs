use std::io;

fn main() {
  let mut input_p1 = String::new();
  let mut input_p2 = String::new();

  io::stdin().read_line(&mut input_p1).unwrap();
  io::stdin().read_line(&mut input_p2).unwrap();

  let p1: Vec<&str> = input_p1.split_whitespace().collect();
  let p2: Vec<&str> = input_p2.split_whitespace().collect();

  let x1 = p1[0].parse::<f64>().unwrap();
  let y1 = p1[1].parse::<f64>().unwrap();

  let x2 = p2[0].parse::<f64>().unwrap();
  let y2 = p2[1].parse::<f64>().unwrap();

  let d = (x2 - x1) *  (x2 - x1) + (y2 - y1) * (y2 - y1);
  println!("{:.4}", d.sqrt());
}