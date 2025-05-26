use std::io::{
  self,
  BufRead,
  BufReader
};
use std::collections::BTreeMap;

fn main() {
  let mut reader = BufReader::new(io::stdin());
  let mut input = String::with_capacity(32);

  reader.read_line(&mut input).unwrap();

  let n: u32 = input.trim()
    .parse()
    .unwrap();

  let mut threes: BTreeMap<String, u32> = BTreeMap::new();
  let mut total = 0;
  let mut counter = 0;

  reader.read_line(&mut input).unwrap();
  while counter < n {
    loop {
      input.clear();
      reader.read_line(&mut input).unwrap();

      let three = input.trim().to_string();

      if three.is_empty() {
        counter += 1;
        break
      }

      total += 1;
      threes.entry(three)
        .and_modify(|curr| *curr += 1)
        .or_insert(1);
    }
    
    for (three, qtd) in &threes {
      println!("{} {:.4}", three, (*qtd as f32 / total as f32) * 100.0);
    }
    total = 0;
    threes.clear();
    if counter != n {
      println!();
    }
  }
  
}