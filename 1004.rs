use std::io;

fn main() {
  let mut input_a = String::new();
  let mut input_b = String::new();

  io::stdin().read_line(&mut input_a).unwrap();
  io::stdin().read_line(&mut input_b).unwrap();

  let a = input_a.trim().parse::<i32>().unwrap();
  let b = input_b.trim().parse::<i32>().unwrap();
  
  println!("PROD = {}", a * b);
}