use std::io;

fn main() {
  let mut input = String::new();

  io::stdin().read_line(&mut input).unwrap();

  let n: u32 = input.trim()
    .parse()
    .unwrap();

  let mut counter = 0;

  for i in 0..1000 {
    println!("N[{}] = {}", i, counter);
    counter += 1;
    if counter == n {
      counter = 0;
    }
  }
}