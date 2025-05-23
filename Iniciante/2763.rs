use std::io;

fn main() {
  let mut input = String::with_capacity(16);

  io::stdin().read_line(&mut input).unwrap();

  println!("{}", &input[0..3]);
  println!("{}", &input[4..7]);
  println!("{}", &input[8..11]);
  println!("{}", &input[12..14]);
}