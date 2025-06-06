use std::io;

fn main() {
  let mut input = String::with_capacity(8);

  io::stdin().read_line(&mut input).unwrap();

  input.clear();

  io::stdin().read_line(&mut input).unwrap();

  let values = input.split_whitespace();

  let mut result = i32::MAX;
  let mut index = 0;

  for (i, value) in values.enumerate() {
    let n: i32 = value.parse().unwrap();
    if n < result {
      result = n;
      index = i;
    }
  }

  println!("Menor valor: {}", result);
  println!("Posicao: {}", index);
}