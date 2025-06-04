use std::io;

fn main() {
  let mut input = String::with_capacity(8);

  io::stdin().read_line(&mut input).unwrap();

  let k: u16 = input.trim()
    .parse()
    .unwrap();

  let n = match k {
    0..=1 => 1,
    2..=3 => 3,
    4..=5 => 5,
    6..=10 => 10,
    11..=25 => 25,
    26..=50 => 50, 
    _ => 100
  };

  println!("Top {}", n);
}