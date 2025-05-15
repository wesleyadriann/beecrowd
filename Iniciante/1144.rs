use std::io;

fn main() {
  let mut input = String::new();

  io::stdin()
    .read_line(&mut input)
    .unwrap();

  let n: u32 = input.trim()
    .parse()
    .unwrap();

  for i in 1..=n {
    let pow_2 = i.pow(2);
    let pow_3 = i.pow(3);
    println!("{} {} {}", i, pow_2, pow_3);
    println!("{} {} {}", i, pow_2 + 1, pow_3 + 1);
  }
}