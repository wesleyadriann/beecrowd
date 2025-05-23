use std::io::{
  self,
  BufRead,
  BufReader,
};

fn main() {
  let mut reader = BufReader::new(io::stdin());
  let mut input = String::with_capacity(8);

  reader.read_line(&mut input).unwrap();

  let c: u32 = input.trim()
    .parse()
    .unwrap();

  for _ in 0..c {
    input.clear();
    reader.read_line(&mut input).unwrap();

    let n: u32 = input.trim()
      .parse()
      .unwrap();

    if n > 8000 {
      println!("Mais de 8000!");
    } else {
      println!("Inseto!");
    }
  }
}