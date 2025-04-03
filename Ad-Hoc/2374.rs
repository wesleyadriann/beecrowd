use std::io;

fn main() {
  let stdin = io::stdin();

  let mut input = String::new();

  stdin.read_line(&mut input).unwrap();
  stdin.read_line(&mut input).unwrap();

  let values: Vec<i16> = input.trim()
    .split('\n')
    .map(|x| x.trim().parse().unwrap())
    .collect();

  let (n, m) = (values[0], values[1]);

  let result: i16 = n - m;

  println!("{}", result);
}