use std::io;

fn main() {
  let mut input = String::new();

  let n: u32 = match io::stdin().read_line(&mut input) {
    Ok(_) => input.trim().parse().unwrap(),
    Err(_) => return,
  };

  let result: u32 = (1..=n).product();

  println!("{}", result);
}