use std::io;

fn main() {
  let mut input = String::new();

  let tomadas: Vec<u16> = match io::stdin().read_line(&mut input) {
    Ok(_) => input.split_whitespace()
      .map(|x| x.parse().unwrap())
      .collect(),
    Err(_) => return
  };

  let mut result = 1;
  for tomada in tomadas {
    result += tomada - 1;
  }

  println!("{}", result);
}