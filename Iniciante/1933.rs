use std::io;

fn main() {
  let mut input = String::with_capacity(8);

  io::stdin().read_line(&mut input).unwrap();

  let mut values = input.split_whitespace();

  let c1: u16 = values.next()
    .and_then(|x| x.parse().ok())
    .unwrap();

  let c2: u16 = values.next()
    .and_then(|x| x.parse().ok())
    .unwrap();

  if c1 == c2 {
    println!("{}", c1);
    return
  }

  let bigger = if c1 > c2 { c1 } else { c2 };

  println!("{}", bigger);
}