use std::io;

fn main() {
  let stdin = io::stdin();

  let mut input = String::new();

  let x: u32 = match stdin.read_line(&mut input) {
    Ok(_) => input.trim()
      .parse()
      .unwrap(),
    Err(_) => return,
  };

  input.clear();
  let y: u32 = match stdin.read_line(&mut input) {
    Ok(_) => input.trim()
      .parse()
      .unwrap(),
    Err(_) => return,
  };

  let (start, end) = if x > y { (y, x) } else { (x, y) };

  let mut result = 0;
  for i in start..=end {
    if i % 13 != 0 {
      result += i;
    }
  }

  println!("{}", result);
}