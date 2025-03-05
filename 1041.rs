use std::io;

fn main() {
  let mut input = String::new();

  io::stdin().read_line(&mut input).unwrap();

  let values: Vec<&str> = input.split_whitespace().collect();

  let x = values[0].parse::<f64>().unwrap();
  let y = values[1].parse::<f64>().unwrap();

  if x == 0.0 && y == 0.0 {
    println!("Origem");
    return;
  } else if x == 0.0 {
    println!("Eixo Y");
    return;
  } else if y == 0.0 {
    println!("Eixo X");
    return;
  }

  let mut result = 1;

  if x < 0.0 && y > 0.0 {
    result = 2;
  } else if x < 0.0 && y < 0.0 {
    result = 3;
  } else if x > 0.0 && y < 0.0 {
    result = 4;
  }

  println!("Q{}", result);
}