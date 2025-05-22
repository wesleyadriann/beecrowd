use std::io;

fn main() {
  let mut input = String::new();

  io::stdin().read_line(&mut input).unwrap();

  let result = match input.trim() {
    "XXO" => "Alice",
    "OXX" => "Alice",
    "XOX" => "*",
    _ => "?",
  };

  println!("{}", result);
}