use std::io;

fn main() {
  const PI: f64 = 3.14159;

  let mut input = String::new();

  io::stdin().read_line(&mut input).unwrap();

  let raio = input.trim().parse::<f64>().unwrap();


  println!("VOLUME = {:.3}", (4.0/3.0) * PI * (raio * raio * raio));
}