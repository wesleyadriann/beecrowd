use std::io::{
  self,
  BufRead,
  BufReader,
};

fn main() {
  let mut reader = BufReader::new(io::stdin());
  let mut input = String::with_capacity(12);

  let n = match reader.read_line(&mut input) {
    Ok(_) => input.trim().parse().unwrap(),
    Err(_) => return
  };

  for _ in 0..n {
    input.clear();
    reader.read_line(&mut input).unwrap();

    let mut values = input.split_whitespace();

    let f1: u32 = values.next()
      .and_then(|x| x.parse().ok())
      .unwrap();

    let f2: u32 = values.next()
      .and_then(|x| x.parse().ok())
      .unwrap();

    println!("{}", mdc(f1, f2));
  }
}

fn mdc(n1: u32, n2: u32) -> u32 {
  let mut a = n1;
  let mut b = n2;
  while b != 0 {
    let r = a % b;
    a = b;
    b = r;
  }
  return a;
}