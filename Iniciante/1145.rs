use std::io;

fn main() {
  let mut input = String::new();

  io::stdin().read_line(&mut input).unwrap();

  let mut values = input.split_whitespace();

  let x: u32 = values.next()
    .and_then(|x| x.parse().ok())
    .unwrap();

  let y: u32 = values.next()
    .and_then(|x| x.parse().ok())
    .unwrap();

  let mut line = String::new();
  let mut counter = 0;
  for i in 1..=y {
    line.push_str(&format!("{} ", i));
    counter += 1;

    if counter == x {
      println!("{}", line.trim());
      line.clear();
      counter = 0;
    }
  }
}