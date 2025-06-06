use std::io::{
  self,
  BufRead,
  BufReader,
};
use std::collections::BTreeMap;

fn main() {
  let mut reader = BufReader::new(io::stdin());

  let mut input = String::with_capacity(202);

  reader.read_line(&mut input).unwrap();

  let n: u32 = input.trim()
    .parse()
    .unwrap_or(0);

  for _ in 0..n {
    input.clear();
    let mut result: BTreeMap<char, u32> = BTreeMap::new();

    reader.read_line(&mut input).unwrap();
    
    let lower = input.trim()
      .to_lowercase();
    let chars = lower.chars();
    
    let mut bigger = 0;
    for c in chars {
      if c.is_alphabetic() {
        result.entry(c)
          .and_modify(|qtd|  {
            let curr = *qtd + 1;
            if curr > bigger  {
              bigger = curr;
            }
            *qtd = curr
          })
          .or_insert(1);
      }
    }

    for (c, qtd) in &result {
      if *qtd >= bigger {
        print!("{}",c);
      }
    }
    println!();
  }
}