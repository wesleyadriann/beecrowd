use std::io::{
  self,
  BufReader,
  BufRead,
};

fn main() {
  let mut reader = BufReader::new(io::stdin());
  let mut input = String::with_capacity(32);

  reader.read_line(&mut input).unwrap();

  let t: u32 = input.trim()
    .parse()
    .unwrap();

  for i in 0..t {
    input.clear();

    reader.read_line(&mut input).unwrap();

    let mut values = input.split_whitespace();

    let qtd: u32 = values.next()
      .and_then(|x| x.parse().ok())
      .unwrap();

    let ages: Vec<&str> = values.collect();
    let mut mid = (qtd as f32 / 2.0).ceil() as usize; 
    if qtd % 2 != 0 {
      mid -= 1;
    }
    
    println!("Case {}: {}", i + 1, ages[mid]);
  }
}