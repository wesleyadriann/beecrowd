use std::io;

fn main() {
  let mut input = String::new();

  io::stdin().read_line(&mut input).unwrap();

  let value = input.trim().parse::<f64>().unwrap();

  let l_bracket = if value <= 25.0 { "[" } else { "(" } ;
  let mut interval = [75, 100];

  if value <= 25.0 {
    interval = [0, 25];
  } else if value <= 50.0 {
    interval = [25, 50];
  } else if value <= 75.0 {
    interval = [50, 75];
  }

  if value < 0.0 || value > 100.0 {
    println!("Fora de intervalo");
  } else {
    println!("Intervalo {}{},{}]", l_bracket, interval[0], interval[1]);
  }
}