use std::io::{self, BufRead};

fn main() {
  let stdin = io::stdin();

  let mut lines = stdin.lock()
    .lines()
    .map(|x| x.unwrap());

  let mut max_index = 0;
  let mut max_value = 0;

  for i in 0..100 {
    let line = lines.next().unwrap();
    let n: u32 = line.parse().unwrap();

    if n >= max_value {
      max_value = n;
      max_index = i;
    }
  }

  println!("{}", max_value);
  println!("{}", max_index + 1);
}