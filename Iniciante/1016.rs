use std::io;

fn main() {
  let mut input = String::new();

  io::stdin().read_line(&mut input).unwrap();

  let d = input.trim().parse::<f32>().unwrap();

  let t = (d / 30.0) * 60.0; 

  println!("{} minutos", t);
}