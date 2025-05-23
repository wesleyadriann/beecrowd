use std::io::{
  self,
  BufRead,
  BufReader,
};

fn main() {
  let mut reader = BufReader::new(io::stdin());
  let mut input = String::with_capacity(32);

  reader.read_line(&mut input).unwrap();

  let c: u32 = input.trim()
    .parse()
    .unwrap();

  for _ in 0..c {
    input.clear();
    reader.read_line(&mut input).unwrap();

    let result: String = input.trim()
      .chars()
      .rev()
      .filter(|c| c.is_lowercase())
      .collect();

    println!("{}", result);
  }
}