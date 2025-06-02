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

  for _ in 0..n {
    input.clear();
    
    reader.read_line(&mut input).unwrap();
    
    let mut values = input.split_whitespace();
    
    let mut x: i32 = values.next()
      .and_then(|x| x.parse().ok())
      .unwrap();
  
    let y: i32 = values.next()
      .and_then(|x| x.parse().ok())
      .unwrap();

    let mut result = 0;
    let mut counter = 0;

    while counter < y {
      if x % 2 != 0 {
        result += x;
        counter += 1;
      }
      x += 1;
    }

    println!("{}", result);
  }
}