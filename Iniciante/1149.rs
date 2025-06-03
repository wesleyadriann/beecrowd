use std::io;

fn main() {
  let mut input = String::new();

  io::stdin().read_line(&mut input).unwrap();

  let mut values = input.split_whitespace();

  let a: i32 = values.next()
    .and_then(|x| x.parse().ok())
    .unwrap();

  let n: i32;
  loop {
    let next = values.next().unwrap();
    if &next[0..1] == "-" || &next[0..1] == "0" {
      continue
    }
    n = next.parse().unwrap();
    break
  }

  let mut result = a;

  for i in 1..n {
    result += a + i;
  }

  println!("{}", result);
}