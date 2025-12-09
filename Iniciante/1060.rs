use std::io;

fn main() {
  let mut input = String::new();

  io::stdin().read_line(&mut input).unwrap();
  io::stdin().read_line(&mut input).unwrap();
  io::stdin().read_line(&mut input).unwrap();
  io::stdin().read_line(&mut input).unwrap();
  io::stdin().read_line(&mut input).unwrap();
  io::stdin().read_line(&mut input).unwrap();

  let values: Vec<f64> = input.trim()
    .split('\n')
    .map(|x| x.trim().parse().unwrap())
    .collect();

  let mut count = 0;

  for number in values {
    if number > 0.0 {
      count += 1;
    }
  }

  println!("{} valores positivos", count);
}
