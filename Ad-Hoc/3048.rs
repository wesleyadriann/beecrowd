use std::io::{
  self,
  BufRead,
  BufReader
};

fn main() {
  let mut reader = BufReader::new(io::stdin());
  let mut input = String::with_capacity(8);

  reader.read_line(&mut input).unwrap();

  let n: u32 = input.trim()
    .parse()
    .unwrap();

  let mut prev = 0;
  let mut total = 0;
  for _ in 0..n {
    input.clear();
    reader.read_line(&mut input).unwrap();

    let v: u32 = input.trim()
      .parse()
      .unwrap();

    if v != prev {
      prev = v;
      total += 1;
    }
  }

  println!("{}", total);
}
