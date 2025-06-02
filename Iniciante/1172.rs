use std::io::{
  self,
  BufRead,
  BufReader
};

fn main() {
  let mut reader = BufReader::new(io::stdin());
  let mut input = String::with_capacity(8);

  for i in 0..10 {
    input.clear();
    reader.read_line(&mut input).unwrap();

    let mut x: i32 = input.trim()
      .parse()
      .unwrap();

    if x < 1 {
      x = 1;
    }

    println!("X[{}] = {}", i, x);
  }
}