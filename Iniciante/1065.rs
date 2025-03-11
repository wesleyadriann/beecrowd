use std::io;

fn main() {
  let mut input = String::new();

  io::stdin().read_line(&mut input).unwrap();
  io::stdin().read_line(&mut input).unwrap();
  io::stdin().read_line(&mut input).unwrap();
  io::stdin().read_line(&mut input).unwrap();
  io::stdin().read_line(&mut input).unwrap();

  let values: Vec<i32> = input.trim()
    .split("\n")
    .map(|x| x.trim().parse().unwrap())
    .collect();

  
  let mut count = 0;

  for value in values {
    if value % 2 == 0 {
      count += 1;
    }
  }

  println!("{} valores pares", count);
}