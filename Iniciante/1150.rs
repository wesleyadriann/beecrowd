use std::io;

fn main() {
  let stdin = io::stdin();

  let mut input = String::with_capacity(16);

  stdin.read_line(&mut input).unwrap();

  let x: i32 = input.trim()
    .parse()
    .unwrap();

  let mut z: i32;

  loop {
    input.clear();
    stdin.read_line(&mut input).unwrap();
    z = input.trim()
      .parse()
      .unwrap();

    if z > x {
      break
    }

  }

  let mut acc = x;
  let mut result = 1;
  while acc <= z {
    acc += x + result;
    result += 1;
  }

  println!("{}", result);
}