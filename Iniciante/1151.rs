use std::io;

fn main() {
  let mut input = String::with_capacity(32);

  io::stdin().read_line(&mut input).unwrap();

  let n: u32 = input.trim()
    .parse()
    .unwrap();

  if n == 1 {
    println!("0");
    return;
  }

  let mut result = String::with_capacity(260);
  result.push_str("0 1");

  let mut a = 0;
  let mut b = 1;

  for _ in 2..n {
    let next = a + b;
    result.push_str(&format!(" {}", next));
    a = b;
    b = next;
  }

  println!("{}", result);
}