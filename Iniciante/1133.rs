use std::io;

fn main() {
  let stdin = io::stdin();

  let mut input = String::new();

  let x: i32 = match stdin.read_line(&mut input) {
    Ok(_) => input.trim()
      .parse()
      .unwrap(),
    Err(_) => return,
  };

  input.clear();
  let y: i32 = match stdin.read_line(&mut input) {
    Ok(_) => input.trim()
      .parse()
      .unwrap(),
    Err(_) => return
  };

  let (start, end) = if x > y { (y + 1, x) } else { (x + 1, y) };

  for i in start..end {
    if i % 5 == 2 || i % 5 == 3 {
      println!("{}", i);
    } 
  }
}