use std::io;

fn main() {
  let mut input = String::new();

  io::stdin().read_line(&mut input).unwrap();

  let vec_input: Vec<&str> = input.split_whitespace().collect();

  let a = vec_input[0].parse::<i32>().unwrap();
  let b = vec_input[1].parse::<i32>().unwrap();
  let c = vec_input[2].parse::<i32>().unwrap();

  let max_ab = (a + b + (a - b).abs()) / 2;

  let bigger = if max_ab > c { max_ab } else { c }; 

  println!("{} eh o maior", bigger);
}