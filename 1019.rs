use std::io;

fn main() {
  let mut input = String::new();

  io::stdin().read_line(&mut input).unwrap();

  let mut seconds = input.trim().parse::<u32>().unwrap();

  let hours = seconds / 3600;
  seconds = seconds % 3600;

  let minutes = seconds / 60;
  seconds = seconds % 60;  

  println!("{}:{}:{}", hours, minutes, seconds);
}