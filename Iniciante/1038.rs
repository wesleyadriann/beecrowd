use std::io;

fn main() {
  let mut input = String::new();

  io::stdin().read_line(&mut input).unwrap();

  let items: Vec<&str> = input.split_whitespace().collect();
  let code = items[0].parse::<u32>().unwrap();
  let qtd = items[1].parse::<f64>().unwrap();
  
  let value = match code {
    1 => 4.0,
    2 => 4.5,
    3 => 5.0,
    4 => 2.0,
    5 => 1.5,
    _ => 0.0,
  };

  println!("Total: R$ {:.2}", value * qtd);
}