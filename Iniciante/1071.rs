use std::{io, cmp};

fn main() {
  let mut input = String::new();
  
  let stdin = io::stdin();

  stdin.read_line(&mut input).unwrap();
  stdin.read_line(&mut input).unwrap();

  let values: Vec<i32> = input.trim()
    .lines()
    .map(|x| x.trim().parse().unwrap())
    .collect();

  let x = cmp::min(values[0], values[1]) + 1;
  let y = cmp::max(values[0], values[1]);


  let mut acc  = 0 ;
  for i in x..y {
    if i % 2 != 0 {
      acc += i;
    }
  }

  println!("{}", acc);
}