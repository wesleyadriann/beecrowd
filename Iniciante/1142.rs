use std::io;

fn main() {
  let mut input = String::new();

  io::stdin().read_line(&mut input).unwrap();

  let n: u32 = input.trim()
    .parse()
    .unwrap();

  let mut count = 1;
  for _ in 0..n {
    println!("{} {} {} PUM", count, count + 1, count + 2);
    count += 4;
  }
}