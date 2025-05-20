use std::io;

fn main() {
  let stdin = io::stdin();

  let mut input = String::with_capacity(32);

  loop {
    input.clear();

    if stdin.read_line(&mut input).unwrap() == 0 {
      break;
    };

    let mut values = input.split_whitespace();

    let x: i32 = values.next()
      .and_then(|x| x.parse().ok())
      .unwrap();

    let y: i32 = values.next()  
      .and_then(|x| x.parse().ok())
      .unwrap();

    println!("{}", 2 * x * y);
  }
}