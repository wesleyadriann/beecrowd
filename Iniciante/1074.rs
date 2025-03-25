use std::io;

fn main() {
  let stdin = io::stdin();

  let mut input = String::new();

  stdin.read_line(&mut input).unwrap();

  let n: u32 = input.trim()
    .parse()
    .unwrap();

  for _ in 0..n {
    input.clear();
    stdin.read_line(&mut input).unwrap();

    let x: i32 = input.trim()
      .parse()
      .unwrap();

    if x == 0 {
      println!("NULL");
      continue;
    }

    let mut result = String::new();

    if x % 2 == 0 {
      result.push_str("EVEN");
    } else {
      result.push_str("ODD");
    }

    if x > 0 {
      result.push_str(" POSITIVE");
    } else {
      result.push_str(" NEGATIVE");
    }

    println!("{}", result);
  }
}