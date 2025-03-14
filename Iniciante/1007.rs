use std::io;

fn main() {
  let mut input_a = String::new();
  let mut input_b = String::new();
  let mut input_c = String::new();
  let mut input_d = String::new();

  io::stdin().read_line(&mut input_a).unwrap();
  io::stdin().read_line(&mut input_b).unwrap();
  io::stdin().read_line(&mut input_c).unwrap();
  io::stdin().read_line(&mut input_d).unwrap();

  let a = input_a.trim().parse::<i32>().unwrap();
  let b = input_b.trim().parse::<i32>().unwrap();
  let c = input_c.trim().parse::<i32>().unwrap();
  let d = input_d.trim().parse::<i32>().unwrap();

  println!("DIFERENCA = {}", a * b - c * d);
}