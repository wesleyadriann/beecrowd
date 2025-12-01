use std::io;

fn main() {
  let mut input = String::new();

  io::stdin().read_line(&mut input).unwrap();

  let mut values: Vec<f64> = input.split_whitespace()
    .map(|x| x.parse().unwrap()) 
    .collect(); 

  values.sort_by(|a, b| b.partial_cmp(a).unwrap()); 

  let a = values[0];
  let b = values[1];
  let c = values[2];

  if a >= b + c {
    println!("NAO FORMA TRIANGULO");
    return
  }

  let pow_a = a * a;
  let pow_b = b * b;
  let pow_c = c * c;

  if pow_a == pow_b + pow_c {
    println!("TRIANGULO RETANGULO");
  } else if pow_a > pow_b + pow_c {
    println!("TRIANGULO OBTUSANGULO");
  } else {
    println!("TRIANGULO ACUTANGULO");
  }

  if a == b && b == c {
    println!("TRIANGULO EQUILATERO");
  } else if a == b || b == c || a == c {
    println!("TRIANGULO ISOSCELES");
  }
}
