use std::io;

fn main() {
  let stdin = io::stdin();

  let mut input = String::new();
  let mut result = 225;

  for i in 0..5 {
    input.clear();
    let value: u32 = match stdin.read_line(&mut input) {
      Ok(_) => input.trim()
        .parse()
        .unwrap(),
      Err(_) => break,
    };

    let multiply = match i {
      0 => 300,
      1 => 1500,
      2 => 600,
      3 => 1000,
      _ => 150,
    };

    result += multiply * value;
  }
  println!("{}", result);
}