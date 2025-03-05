use std::io;

fn main() {
  let mut input = String::new();

  io::stdin().read_line(&mut input).unwrap();

  let values: Vec<&str> = input.split_whitespace().collect();

  let a = values[0].parse::<u32>().unwrap();
  let b = values[1].parse::<u32>().unwrap();

  let is = if a > b { a % b == 0 } else { b % a == 0 };

  if is {
    println!("Sao Multiplos");
  } else {
    println!("Nao sao Multiplos");
  }

}