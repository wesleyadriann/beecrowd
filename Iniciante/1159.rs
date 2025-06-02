use std::io::{
  self,
  BufRead,
  BufReader
};

fn main() {
  let mut reader = BufReader::new(io::stdin());
  let mut input = String::with_capacity(8);

  loop {
    input.clear();

    reader.read_line(&mut input).unwrap();

    if input.trim() == "0" {
      break
    }
    
    let mut x: i32 = input.trim()
      .parse()
      .unwrap();

    let mut result = 0;
    let mut counter = 0;

    while counter < 5 {
      if x % 2 == 0{
        result += x;
        counter += 1;
      }
      x += 1;
    }

    println!("{}", result);
  }
}