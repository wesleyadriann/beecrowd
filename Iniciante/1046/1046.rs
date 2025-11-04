use std::io;

fn main() {
  let mut input = String::new();

  io::stdin().read_line(&mut input).unwrap();

  let values: Vec<u16> = input.split_whitespace()
    .map(|x| x.parse().unwrap())
    .collect();

  let h1 = values[0];
  let h2 = values[1];

  if h1 >= h2 {
    println!("O JOGO DUROU {} HORA(S)", 24 - (h1 - h2));
  } else {
    println!("O JOGO DUROU {} HORA(S)", h2 - h1);
  }
}
