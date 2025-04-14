use std::io;

fn main() {
  let mut input = String::new();

  let values: Vec<u8> = match io::stdin().read_line(&mut input) {
    Ok(_) => input.split_whitespace()
      .map(|x| x.parse().unwrap())
      .collect(),
    Err(_) => return,
  };

  let (p, r) = (values[0], values[1]);

  if p == 0 {
    println!("C");
    return
  }

  if r == 0 {
    println!("B");
  } else {
    println!("A");
  }
}