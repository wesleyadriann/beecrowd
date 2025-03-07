use std::io;

fn main() {
  const PI: f64 = 3.14159;

  let mut input = String::new();

  io::stdin().read_line(&mut input).unwrap();

  let vec_input: Vec<&str> = input.split_whitespace().collect();

  let value_a = vec_input[0].parse::<f64>().unwrap();
  let value_b = vec_input[1].parse::<f64>().unwrap();
  let value_c = vec_input[2].parse::<f64>().unwrap();
   
  println!("TRIANGULO: {:.3}
CIRCULO: {:.3}
TRAPEZIO: {:.3}
QUADRADO: {:.3}
RETANGULO: {:.3}",
value_a * value_c / 2.0,
value_c * value_c * PI,
(value_a + value_b) * value_c / 2.0,
value_b *  value_b,
value_a * value_b)
}